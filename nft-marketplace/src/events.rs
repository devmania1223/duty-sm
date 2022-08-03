elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::structs::{Auction, AuctionType};

#[allow(clippy::too_many_arguments)]
#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_create_auction_event(self, auction_id: u64, auction: Auction<Self::Api>) {
        self.create_auction_event(
            &auction.auctioned_tokens.token_identifier,
            auction.auctioned_tokens.token_nonce,
            auction_id,
            &auction.auctioned_tokens.amount,
            &auction.original_owner,
            &auction.min_bid,
            &auction.max_bid.unwrap_or_else(BigUint::zero),
            auction.start_time,
            auction.deadline,
            auction.payment_token,
            auction.payment_nonce,
            auction.auction_type,
            auction.creator_royalties_percentage,
        )
    }

    fn emit_end_auction_event(self, auction_id: u64, auction: Auction<Self::Api>) {
        self.end_auction_event(
            &auction.auctioned_tokens.token_identifier,
            auction.auctioned_tokens.token_nonce,
            auction_id,
            &auction.auctioned_tokens.amount,
            &auction.current_winner,
            &auction.current_bid,
        );
    }

    fn emit_cancel_auction_event(self, auction_id: u64, auction: Auction<Self::Api>) {
        self.cancel_auction_event(
            &auction.auctioned_tokens.token_identifier,
            auction.auctioned_tokens.token_nonce,
            auction_id,
            &auction.auctioned_tokens.amount,
            &auction.original_owner,
        );
    }

    fn emit_bid_event(self, auction_id: u64, auction: Auction<Self::Api>) {
        self.bid_event(
            &auction.auctioned_tokens.token_identifier,
            auction.auctioned_tokens.token_nonce,
            auction_id,
            &auction.auctioned_tokens.amount,
            &auction.current_winner,
            &auction.current_bid,
        );
    }

    fn emit_buy_event(
        self,
        auction_id: u64,
        auction: Auction<Self::Api>,
    ) {
        self.buy_event(
            &auction.auctioned_tokens.token_identifier,
            auction.auctioned_tokens.token_nonce,
            auction_id,
            &auction.current_winner,
            &auction.min_bid,
        );
    }


    #[event("create_auction_event")]
    fn create_auction_event(
        &self,
        #[indexed] create_auction_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] auctioned_token_amount: &BigUint,
        #[indexed] seller: &ManagedAddress,
        #[indexed] min_bid: &BigUint,
        #[indexed] max_bid: &BigUint,
        #[indexed] start_time: u64,
        #[indexed] deadline: u64,
        #[indexed] accepted_payment_token: EgldOrEsdtTokenIdentifier,
        #[indexed] accepted_payment_token_nonce: u64,
        #[indexed] auction_type: AuctionType,
        creator_royalties_percentage: BigUint, // between 0 and 10,000
    );

    #[event("end_auction_event")]
    fn end_auction_event(
        &self,
        #[indexed] create_auction_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] nr_auctioned_tokens: &BigUint,
        #[indexed] auction_winner: &ManagedAddress,
        #[indexed] winning_bid_amount: &BigUint,
    );

    #[event("cancel_auction_event")]
    fn cancel_auction_event(
        &self,
        #[indexed] create_auction_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] nr_auctioned_tokens: &BigUint,
        #[indexed] seller: &ManagedAddress,
    );

    #[event("bid_event")]
    fn bid_event(
        &self,
        #[indexed] create_auction_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] nr_auctioned_tokens: &BigUint,
        #[indexed] bidder: &ManagedAddress,
        #[indexed] bid_amount: &BigUint,
    );

    #[event("buy_event")]
    fn buy_event(
        &self,
        #[indexed] create_auction_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] buyer: &ManagedAddress,
        #[indexed] bid_nft_amount: &BigUint,
    );    
}
