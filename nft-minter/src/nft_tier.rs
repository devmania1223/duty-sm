elrond_wasm::imports!();

use crate::common_storage::BrandId;

const VEC_MAPPER_FIRST_ITEM_INDEX: usize = 1;

#[elrond_wasm::module]
pub trait NftTierModule {
    fn get_next_random_id(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> UniqueId {
        let mut id_mapper = self.available_ids(brand_id);
        let last_id_index = id_mapper.len();
        require!(last_id_index > 0, "No more NFTs available for brand");

        let rand_index = self.get_random_usize(VEC_MAPPER_FIRST_ITEM_INDEX, last_id_index + 1);
        let rand_id = id_mapper.swap_remove(rand_index);

        rand_id
    }

    fn verify_nft_id(
        &self,
        brand_id: &BrandId<Self::Api>,
        nft_id: usize,
    ) -> UniqueId {
        let mut id_mapper = self.available_ids(brand_id);
        let last_id_index = id_mapper.len();
        let mut id = 0;
        for i in 0..last_id_index - 1 {
            if id_mapper.get(i) == nft_id {
                id = id_mapper.swap_remove(i);
            }
        }

        require!(id > 0, "Can not find the Nft id");

        id
    }

    /// range is [min, max)
    fn get_random_usize(&self, min: usize, max: usize) -> usize {
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        rand_source.next_usize_in_range(min, max)
    }


    #[storage_mapper("availableIds")]
    fn available_ids(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> UniqueIdMapper<Self::Api>;

    #[storage_mapper("totalNfts")]
    fn total_nfts(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> SingleValueMapper<usize>;
}
