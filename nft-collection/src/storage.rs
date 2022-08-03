elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::structs::{CollectionId, CollectionInfo, MintPrice,
    CollectionHash,Tag, TempCallbackStorageInfo};

#[elrond_wasm::module]
pub trait StorageModule:
{ 
    #[view(getRegisterdCollectionHashes)]
    #[storage_mapper("registeredCollectionHashes")]
    fn registered_collection_hashes(&self) -> UnorderedSetMapper<CollectionHash<Self::Api>>;

    #[view(getRegisteredCollections)]
    #[storage_mapper("registeredCollections")]
    fn registered_collections(&self) -> UnorderedSetMapper<CollectionId<Self::Api>>;

    #[view(getNftTokenIdForCollection)]
    #[storage_mapper("nftTokenId")]
    fn nft_token(&self, collection_id: &CollectionId<Self::Api>) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("collectionInfo")]
    fn collection_info(&self, collection_id: &CollectionId<Self::Api>) -> SingleValueMapper<CollectionInfo<Self::Api>>;

    #[view(getPriceForTier)]
    #[storage_mapper("priceForTier")]
    fn price_for_tier(
        &self,
        collection_id: &CollectionId<Self::Api>,
    ) -> SingleValueMapper<MintPrice<Self::Api>>;

    #[view(getTagsForCollection)]
    #[storage_mapper("tagsForCollection")]
    fn tags_for_collection(
        &self,
        collection_id: &CollectionId<Self::Api>,
    ) -> SingleValueMapper<ManagedVec<Tag<Self::Api>>>;

    #[view(getMintWhitelist)]
    #[storage_mapper("mintWhitelist")]
    fn mint_whitelist(&self, collection_id: &CollectionId<Self::Api>) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getRoyaltiesClaimAddress)]
    #[storage_mapper("royaltiesClaimAddress")]
    fn royalties_claim_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getMintPaymentsClaimAddress)]
    #[storage_mapper("mintPaymentsClaimAddress")]
    fn mint_payments_claim_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getAccumulatedRoyalties)]
    #[storage_mapper("accumulatedRoyalties")]
    fn accumulated_royalties(&self) -> MapMapper<EgldOrEsdtTokenIdentifier, BigUint>;

    #[view(getAccumulatedMintPayments)]
    #[storage_mapper("accumulatedMintPayments")]
    fn accumulated_mint_payments(&self) -> MapMapper<EgldOrEsdtTokenIdentifier, BigUint>;

    #[storage_mapper("adminWhitelist")]
    fn admin_whitelist(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;

    #[storage_mapper("availableIds")]
    fn available_ids(
        &self,
        collection_id: &CollectionId<Self::Api>,
    ) -> UniqueIdMapper<Self::Api>;

    #[storage_mapper("totalNfts")]
    fn total_nfts(
        &self,
        collection_id: &CollectionId<Self::Api>,
    ) -> SingleValueMapper<usize>;

    #[storage_mapper("temporaryCallbackStorage")]
    fn temporary_callback_storage(
        &self,
        collection_id: &CollectionId<Self::Api>,
    ) -> SingleValueMapper<TempCallbackStorageInfo<Self::Api>>;
}
