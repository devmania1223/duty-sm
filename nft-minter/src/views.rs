elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
    brand_creation::INVALID_BRAND_ID_ERR_MSG,
    common_storage::{BrandId, BrandInfo, MintPrice},
};

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct BrandInfoViewResultType<M: ManagedTypeApi> {
    pub brand_id: BrandId<M>,
    pub nft_token_id: TokenIdentifier<M>,
    pub brand_info: BrandInfo<M>,
    pub tier_info: TierInfoEntry<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Debug, PartialEq)]
pub struct TierInfoEntry<M: ManagedTypeApi> {
    pub total_nfts: usize,
    pub available_nfts: usize,
    pub mint_price: MintPrice<M>,
}

#[elrond_wasm::module]
pub trait ViewsModule:
    crate::common_storage::CommonStorageModule + crate::nft_tier::NftTierModule
{
    #[view(getBrandInfo)]
    fn get_brand_info_view(
        &self,
        brand_id: BrandId<Self::Api>,
    ) -> BrandInfoViewResultType<Self::Api> {
        require!(
            self.registered_brands().contains(&brand_id),
            INVALID_BRAND_ID_ERR_MSG
        );

        let nft_token_id = self.nft_token(&brand_id).get_token_id();
        let brand_info = self.brand_info(&brand_id).get();

        let total_nfts = self.total_nfts(&brand_id).get();
        let available_nfts = self.available_ids(&brand_id).len();
        let mint_price = self.price_for_tier(&brand_id).get();

        BrandInfoViewResultType {
            brand_id,
            nft_token_id,
            brand_info,
            tier_info: TierInfoEntry {
                total_nfts,
                available_nfts,
                mint_price,
            },
        }
    }

    #[view(getAllBrandsInfo)]
    fn get_all_brands_info(&self) -> MultiValueEncoded<BrandInfoViewResultType<Self::Api>> {
        let mut result = MultiValueEncoded::new();
        for brand_id in self.registered_brands().iter() {
            let brand_info_entry = self.get_brand_info_view(brand_id);
            result.push(brand_info_entry);
        }

        result
    }
}
