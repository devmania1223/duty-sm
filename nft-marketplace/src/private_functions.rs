elrond_wasm::imports!();

use crate::structs::{Auction, BidSplitAmounts,
    PERCENTAGE_TOTAL, NFT_AMOUNT };

#[elrond_wasm::module]
pub trait PrivateFunctionsModule:
    crate::views::ViewsModule
    + crate::events::EventsModule
{   
    fn is_valid_bid(
        &self,
        auction: &Auction<Self::Api>,
        nft_type: &TokenIdentifier,
        nft_nonce: u64,
        payment_token: &EgldOrEsdtTokenIdentifier,
        payment_nonce: u64,
    ) {
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();

        require!(
            &auction.auctioned_tokens.token_identifier == nft_type
                && auction.auctioned_tokens.token_nonce == nft_nonce,
            "Auction ID does not match the token"
        );
        require!(
            auction.original_owner != caller,
            "Can't bid on your own token"
        );
        require!(
            current_time >= auction.start_time,
            "Auction hasn't started yet"
        );
        require!(current_time < auction.deadline, "Auction ended already");
        require!(
            payment_token == &auction.payment_token && payment_nonce == auction.payment_nonce,
            "Wrong token used as payment"
        );
    }

    fn get_nft_info(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> EsdtTokenData<Self::Api> {
        self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            nft_type,
            nft_nonce,
        )
    }

    fn try_set_fee_percentage(&self, new_fee_percentage: u64) {
        require!(
            new_fee_percentage > 0 && new_fee_percentage < PERCENTAGE_TOTAL,
            "Invalid percentage value, should be between 0 and 10,000"
        );

        self.fee_percentage()
            .set(&BigUint::from(new_fee_percentage));
    }

    fn calculate_amount(&self, total_amount: &BigUint, fee_percentage: &BigUint) -> BigUint {
        total_amount * fee_percentage / PERCENTAGE_TOTAL
    }

    fn calculate_winning_bid_split(
        &self,
        auction: &Auction<Self::Api>,
    ) -> BidSplitAmounts<Self::Api> {
        let creator_royalties =
            self.calculate_amount(&auction.current_bid, &auction.creator_royalties_percentage);
        let fee_amount =
            self.calculate_amount(&auction.current_bid, &auction.marketplace_fee_percentage);
        let mut seller_amount_to_send = auction.current_bid.clone();
        seller_amount_to_send -= &creator_royalties;
        seller_amount_to_send -= &fee_amount;

        BidSplitAmounts {
            creator: creator_royalties,
            marketplace: fee_amount,
            seller: seller_amount_to_send,
        }
    }

    fn distribute_tokens_after_auction_end(
        &self,
        auction: &Auction<Self::Api>,
    ) {
        let nft_type = &auction.auctioned_tokens.token_identifier;
        let nft_nonce = auction.auctioned_tokens.token_nonce;

        if !auction.current_winner.is_zero() {
            let nft_info = self.get_nft_info(nft_type, nft_nonce);
            let token_id = &auction.payment_token;
            let nonce = auction.payment_nonce;
            let bid_split_amounts = self.calculate_winning_bid_split(auction);

            // send part as fee for contract owner
            let owner = self.blockchain().get_owner_address();
            self.transfer_or_save_payment(&owner, token_id, nonce, &bid_split_amounts.marketplace);

            // send part as royalties to creator
            self.transfer_or_save_payment(
                &nft_info.creator,
                token_id,
                nonce,
                &bid_split_amounts.creator,
            );

            // send rest of the bid to original owner
            self.transfer_or_save_payment(
                &auction.original_owner,
                token_id,
                nonce,
                &bid_split_amounts.seller,
            );

            // send NFT to auction winner
            let nft_amount = BigUint::from(NFT_AMOUNT);
            self.transfer_or_save_payment(
                &auction.current_winner,
                &EgldOrEsdtTokenIdentifier::esdt(nft_type.clone()),
                nft_nonce,
                &nft_amount,
            );
        } else {
            // return to original owner
            self.transfer_or_save_payment(
                &auction.original_owner,
                &EgldOrEsdtTokenIdentifier::esdt(nft_type.clone()),
                nft_nonce,
                &auction.auctioned_tokens.amount,
            );
        }
    }

    fn transfer_or_save_payment(
        &self,
        to: &ManagedAddress,
        token_id: &EgldOrEsdtTokenIdentifier,
        nonce: u64,
        amount: &BigUint,
    ) {
        if amount == &0 {
            return;
        }

        if self.blockchain().is_smart_contract(to) {
            self.claimable_amount(to, token_id, nonce)
                .update(|amt| *amt += amount);
        } else {
            self.send().direct(to, token_id, nonce, amount);
        }
    }
}
