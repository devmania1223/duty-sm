#![no_std]

elrond_wasm::imports!();
use crate::structs::{Auction, AuctionType, PERCENTAGE_TOTAL, NFT_AMOUNT};

pub mod events;
pub mod private_functions;
pub mod structs;
pub mod views;

#[elrond_wasm::contract]
pub trait DutyNftMarketplace:
    views::ViewsModule
    + events::EventsModule
    + crate::private_functions::PrivateFunctionsModule
{
    #[init]
    fn init(&self, fee_percentage: u64) {
        self.try_set_fee_percentage(fee_percentage);
    }

    #[only_owner]
    #[endpoint(setFeePercentage)]
    fn set_fee_percentage(&self, new_fee_percentage: u64) {
        self.try_set_fee_percentage(new_fee_percentage);
    }

    #[payable("*")]
    #[endpoint(createAuction)]
    #[allow(clippy::too_many_arguments)]
    fn create_auction(
        &self,
        min_bid: BigUint,
        max_bid: BigUint,
        deadline: u64,
        accepted_payment_token: EgldOrEsdtTokenIdentifier,
        opt_is_fixed_price: OptionalValue<bool>,
        opt_accepted_payment_token_nonce: OptionalValue<u64>,
        opt_start_time: OptionalValue<u64>,
    ) -> u64 {
        let (nft_type, nft_nonce, nft_amount) = self.call_value().single_esdt().into_tuple();

        let current_time = self.blockchain().get_block_timestamp();
        let start_time = match opt_start_time {
            OptionalValue::Some(0) => current_time,
            OptionalValue::Some(st) => st,
            OptionalValue::None => current_time,
        };
        let is_fixed_price = opt_is_fixed_price
            .into_option()
            .unwrap_or_default();

        if is_fixed_price {
            require!(
                min_bid == max_bid,
                "Price must be fixed for this type of auction (min bid equal to max bid)"
            );
        }

        let opt_max_bid = if max_bid > 0u32 {
            require!(min_bid <= max_bid, "Min bid can't be higher than max bid");

            Some(max_bid)
        } else {
            None
        };

        require!(min_bid > 0, "Min bid must be higher than 0");
        require!(nft_amount == NFT_AMOUNT, "NFT amount must be one");
        require!(
            nft_nonce > 0,
            "Only Semi-Fungible and Non-Fungible tokens can be auctioned"
        );
        require!(deadline > current_time, "Deadline can't be in the past");
        require!(
            start_time >= current_time && start_time < deadline,
            "Invalid start time"
        );

        let marketplace_fee_percentage = self.fee_percentage().get();
        let creator_royalties_percentage = self.get_nft_info(&nft_type, nft_nonce).royalties;

        require!(
            &marketplace_fee_percentage + &creator_royalties_percentage < PERCENTAGE_TOTAL,
            "Marketplace fee plus royalties exceeds 100%"
        );

        let accepted_payment_nft_nonce = if accepted_payment_token.is_egld() {
            0
        } else {
            opt_accepted_payment_token_nonce
                .into_option()
                .unwrap_or_default()
        };

        let auction_id = self.last_valid_auction_id().get() + 1;
        self.last_valid_auction_id().set(&auction_id);

        let auction_type = match is_fixed_price {
            true => AuctionType::FixedType,
            false => AuctionType::BidType,
        };

        let auction = Auction {
            auctioned_tokens: EsdtTokenPayment::new(nft_type, nft_nonce, nft_amount),
            auction_type,
            payment_token: accepted_payment_token,
            payment_nonce: accepted_payment_nft_nonce,
            min_bid,
            max_bid: opt_max_bid,
            start_time,
            deadline,
            original_owner: self.blockchain().get_caller(),
            current_bid: BigUint::zero(),
            current_winner: ManagedAddress::zero(),
            marketplace_fee_percentage,
            creator_royalties_percentage,
        };
        self.auction_by_id(auction_id).set(&auction);

        self.emit_create_auction_event(auction_id, auction);

        auction_id
    }

    #[endpoint(endAuction)]
    fn end_auction(&self, auction_id: u64) {
        let auction = self.try_get_auction(auction_id);
        let current_time = self.blockchain().get_block_timestamp();

        let deadline_reached = current_time > auction.deadline;
        let max_bid_reached = if let Some(max_bid) = &auction.max_bid {
            &auction.current_bid == max_bid
        } else {
            false
        };

        require!(
            deadline_reached || max_bid_reached,
            "Auction deadline has not passed nor is the current bid equal to max bid"
        );
        require!(
            auction.auction_type != AuctionType::FixedType,
            "Cannot end this type of auction"
        );

        self.distribute_tokens_after_auction_end(&auction);
        self.auction_by_id(auction_id).clear();

        self.emit_end_auction_event(auction_id, auction);
    }

    #[endpoint(cancelAuction)]
    fn cancel_auction(&self, auction_id: u64) {
        let auction = self.try_get_auction(auction_id);
        let caller = self.blockchain().get_caller();

        require!(
            auction.original_owner == caller,
            "Only the original owner can cancel_auction"
        );
        require!(
            auction.current_bid == 0 || auction.auction_type == AuctionType::FixedType,
            "Can't cancel_auction, NFT already has bids"
        );

        self.auction_by_id(auction_id).clear();

        let nft_type = &auction.auctioned_tokens.token_identifier;
        let nft_nonce = auction.auctioned_tokens.token_nonce;
        let nft_amount = &auction.auctioned_tokens.amount;
        self.transfer_or_save_payment(
            &caller,
            &EgldOrEsdtTokenIdentifier::esdt(nft_type.clone()),
            nft_nonce,
            nft_amount,
        );

        self.emit_cancel_auction_event(auction_id, auction);
    }

    #[payable("*")]
    #[endpoint]
    fn bid(&self, auction_id: u64, nft_type: TokenIdentifier, nft_nonce: u64) {
        let (payment_token, payment_token_nonce, payment_amount) =
            self.call_value().egld_or_single_esdt().into_tuple();
        let mut auction = self.try_get_auction(auction_id);
        let caller = self.blockchain().get_caller();

        self.is_valid_bid(
            &auction,
            &nft_type,
            nft_nonce,
            &payment_token,
            payment_token_nonce,
        );

        require!(
            auction.auction_type != AuctionType::FixedType,
            "Cannot bid on this type of auction"
        );
        require!(auction.current_winner != caller, "Can't outbid yourself");
        require!(
            payment_amount >= auction.min_bid,
            "Bid must be higher than or equal to the min bid"
        );
        require!(
            payment_amount > auction.current_bid,
            "Bid must be higher than the current winning bid"
        );

        if let Some(max_bid) = &auction.max_bid {
            require!(
                &payment_amount <= max_bid,
                "Bid must be less than or equal to the max bid"
            );
        }

        // refund losing bid
        if auction.current_winner != ManagedAddress::zero() {
            self.transfer_or_save_payment(
                &auction.current_winner,
                &auction.payment_token,
                auction.payment_nonce,
                &auction.current_bid,
            );
        }

        // update auction bid and winner
        auction.current_bid = payment_amount;
        auction.current_winner = caller;
        self.auction_by_id(auction_id).set(&auction);

        self.emit_bid_event(auction_id, auction);
    }

    #[payable("*")]
    #[endpoint(buy)]
    fn buy(
        &self,
        auction_id: u64,
        nft_type: TokenIdentifier,
        nft_nonce: u64,
    ) {
        let (payment_token, payment_token_nonce, payment_amount) =
            self.call_value().egld_or_single_esdt().into_tuple();
        let mut auction = self.try_get_auction(auction_id);
        let caller = self.blockchain().get_caller();

        self.is_valid_bid(
            &auction,
            &nft_type,
            nft_nonce,
            &payment_token,
            payment_token_nonce,
        );

        require!(
            auction.auction_type == AuctionType::FixedType,
            "Cannot buy NFT for this type of auction"
        );
 
        require!(
            &auction.min_bid == &payment_amount,
            "Wrong amount paid, must pay equal to the selling price"
        );

        auction.current_winner = caller;
        auction.current_bid = payment_amount;
        self.distribute_tokens_after_auction_end(&auction);

        self.auction_by_id(auction_id).clear();

        self.emit_buy_event(auction_id, auction);
    }

    #[endpoint(claimTokens)]
    fn claim_tokens(
        &self,
        claim_destination: ManagedAddress,
        token_nonce_pairs: MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, u64>>,
    ) -> MultiValue2<BigUint, ManagedVec<EsdtTokenPayment<Self::Api>>> {
        let caller = self.blockchain().get_caller();
        let mut egld_payment_amount = BigUint::zero();
        let mut output_payments = ManagedVec::new();

        for pair in token_nonce_pairs {
            let (token_id, token_nonce) = pair.into_tuple();
            let amount_mapper = self.claimable_amount(&caller, &token_id, token_nonce);
            let amount = amount_mapper.get();

            if amount > 0 {
                amount_mapper.clear();

                if token_id.is_egld() {
                    egld_payment_amount = amount;
                } else {
                    output_payments.push(EsdtTokenPayment::new(
                        token_id.unwrap_esdt(),
                        token_nonce,
                        amount,
                    ));
                }
            }
        }

        if egld_payment_amount > 0 {
            self.send()
                .direct_egld(&claim_destination, &egld_payment_amount);
        }
        if !output_payments.is_empty() {
            self.send()
                .direct_multi(&claim_destination, &output_payments);
        }

        (egld_payment_amount, output_payments).into()
    }    
}
