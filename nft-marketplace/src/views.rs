elrond_wasm::imports!();

use crate::structs::{Auction};

#[elrond_wasm::module]
pub trait ViewsModule { 
    #[view(getFullAuctionData)]
    fn try_get_auction(&self, auction_id: u64) -> Auction<Self::Api> {
        let auction_mapper = self.auction_by_id(auction_id);
        require!(!auction_mapper.is_empty(), "Auction does not exist");
        auction_mapper.get()
    }

    #[view(getLastValidAuctionId)]
    #[storage_mapper("lastValidAuctionId")]
    fn last_valid_auction_id(&self) -> SingleValueMapper<u64>;

    #[view(getMarketplaceFeePercentage)]
    #[storage_mapper("feePercentage")]
    fn fee_percentage(&self) -> SingleValueMapper<BigUint>;

    #[view(getClaimableAmount)]
    #[storage_mapper("claimableAmount")]
    fn claimable_amount(
        &self,
        address: &ManagedAddress,
        token_id: &EgldOrEsdtTokenIdentifier,
        token_nonce: u64,
    ) -> SingleValueMapper<BigUint>;

    #[storage_mapper("auctionById")]
    fn auction_by_id(&self, auction_id: u64) -> SingleValueMapper<Auction<Self::Api>>;   
}
