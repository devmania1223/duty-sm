elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{structs::CollectionId};

#[elrond_wasm::module]
pub trait EventsModule {
    #[event("collectionCreated")]
    fn collection_created_event(
        &self,
        #[indexed] collection_id: &CollectionId<Self::Api>,
        #[indexed] nft_token_id: &TokenIdentifier,
    );

    #[event("nftBought")]
    fn nft_bought_event(
        &self,
        #[indexed] buyer_address: &ManagedAddress,
        #[indexed] collection_id: &CollectionId<Self::Api>,
        nr_nfts_bought: usize,
    );

    #[event("nftGiveaway")]
    fn nft_giveaway_event(
        &self,
        #[indexed] collection_id: &CollectionId<Self::Api>,
        total_nfts_given: usize,
    );

    #[event("nftClaimed")]
    fn nft_claimed_event(
        &self,
        #[indexed] collection_id: &CollectionId<Self::Api>,
        total_nfts_claimed: usize,
    );
}
