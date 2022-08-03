elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::structs::{CollectionId, CollectionInfoViewResultType, TierInfoEntry,
    INVALID_COLLECTION_ID_ERR_MSG };

#[elrond_wasm::module]
pub trait ViewsModule:
    crate::private_functions::PrivateFunctionsModule
    +crate::storage::StorageModule
{
    #[view(getCollectionInfo)]
    fn get_collection_info_view(
        &self,
        collection_id: CollectionId<Self::Api>,
    ) -> CollectionInfoViewResultType<Self::Api> {
        require!(
            self.registered_collections().contains(&collection_id),
            INVALID_COLLECTION_ID_ERR_MSG
        );

        let nft_token_id = self.nft_token(&collection_id).get_token_id();
        let collection_info = self.collection_info(&collection_id).get();

        let total_nfts = self.total_nfts(&collection_id).get();
        let available_nfts = self.available_ids(&collection_id).len();
        let mint_price = self.price_for_tier(&collection_id).get();

        CollectionInfoViewResultType {
            collection_id,
            nft_token_id,
            collection_info,
            tier_info: TierInfoEntry {
                total_nfts,
                available_nfts,
                mint_price,
            },
        }
    }

    #[view(getAllCollectionsInfo)]
    fn get_all_collections_info(&self) -> MultiValueEncoded<CollectionInfoViewResultType<Self::Api>> {
        let mut result = MultiValueEncoded::new();
        for collection_id in self.registered_collections().iter() {
            let collection_info_entry = self.get_collection_info_view(collection_id);
            result.push(collection_info_entry);
        }

        result
    }
}
