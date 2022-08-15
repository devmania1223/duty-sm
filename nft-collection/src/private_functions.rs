elrond_wasm::imports!();

use crate::structs::{CollectionId, CollectionInfo, CollectionHash, GenericAttributes, Uri, MediaType,
    EgldValuePaymentsVecPair, 
    ATTRIBUTES_SEPARATOR, MAX_MEDIA_TYPE_LEN, SUPPORTED_MEDIA_TYPES,
    NFT_AMOUNT, VEC_MAPPER_FIRST_ITEM_INDEX};

#[elrond_wasm::module]
pub trait PrivateFunctionsModule:
  crate::storage::StorageModule {   
    fn require_caller_is_admin(&self) {
        let caller = self.blockchain().get_caller();
        let sc_owner = self.blockchain().get_owner_address();
        if caller == sc_owner {
            return;
        }

        self.admin_whitelist().require_whitelisted(&caller);
    }

    fn build_nft_attributes(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
        collection_id: &CollectionId<Self::Api>,
        nft_id: UniqueId,
    ) -> GenericAttributes<Self::Api> {
        let mut attributes = self.build_attributes_metadata_part(collection_hash, nft_id);
        let tags_attributes = self.tags_for_collection(collection_id).get();
        if !tags_attributes.is_empty() {
            attributes.append_bytes(ATTRIBUTES_SEPARATOR);
            attributes.append(&tags_attributes);
        }

        attributes
    }

    fn build_attributes_metadata_part(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
        nft_id: UniqueId,
    ) -> GenericAttributes<Self::Api> {
        sc_format!(
            "metadata:{}/{}.json",
            collection_hash.as_managed_buffer(),
            nft_id
        )
    }

    fn build_nft_main_file_uri(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
        nft_id: UniqueId,
        media_type: &MediaType<Self::Api>,
    ) -> Uri<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}/{}.{}",
            collection_hash.as_managed_buffer(),
            nft_id,
            media_type
        )
    }

    fn build_nft_json_file_uri(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
        nft_id: UniqueId,
    ) -> Uri<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}/{}.json",
            collection_hash.as_managed_buffer(),
            nft_id,
        )
    }

    fn build_collection_json_file_uri(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
    ) -> Uri<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}/collection.json",
            collection_hash.as_managed_buffer(),
        )
    }

    fn is_supported_media_type(&self, media_type: &MediaType<Self::Api>) -> bool {
        let media_type_len = media_type.len();
        if media_type_len > MAX_MEDIA_TYPE_LEN {
            return false;
        }

        let mut media_static_buffer = [0u8; MAX_MEDIA_TYPE_LEN];
        let slice = &mut media_static_buffer[..media_type_len];
        let _ = media_type.load_slice(0, slice);

        // clippy is wrong, using `slice` directly causes an error
        #[allow(clippy::redundant_slicing)]
        SUPPORTED_MEDIA_TYPES.contains(&&slice[..])
    }   
    
    fn _mint_and_send_random_nft(
        &self,
        to: &ManagedAddress,
        collection_id: &CollectionId<Self::Api>,
        collection_info: &CollectionInfo<Self::Api>,
        nfts_to_send: usize,
    ) -> MultiValueEncoded<MultiValue3<TokenIdentifier, u64, usize>> {
        require!(
            !self.blockchain().is_smart_contract(to),
            "Only user accounts are allowed to mint"
        );

        let total_available_nfts = self.available_ids(collection_id).len();
        require!(
            nfts_to_send <= total_available_nfts,
            "Not enough NFTs available"
        );

        let nft_token_id = self.nft_token(collection_id).get_token_id();
        let mut nft_output_payments = ManagedVec::new();
        let mut output_nfts = MultiValueEncoded::new();
        for _ in 0..nfts_to_send {
            let nft_id = self.get_next_random_id(collection_id);
            let nft_uri = self.build_nft_main_file_uri(
                &collection_info.collection_hash,
                nft_id,
                &collection_info.media_type,
            );
            let nft_json = self.build_nft_json_file_uri(&collection_info.collection_hash, nft_id);
            let collection_json = self.build_collection_json_file_uri(&collection_info.collection_hash);

            let mut uris = ManagedVec::new();
            uris.push(nft_uri);
            uris.push(nft_json);
            uris.push(collection_json);

            let attributes =
                self.build_nft_attributes(&collection_info.collection_hash, collection_id, nft_id);
            let nft_amount = BigUint::from(NFT_AMOUNT);
            let nft_nonce = self.send().esdt_nft_create(
                &nft_token_id,
                &nft_amount,
                &collection_info.token_display_name,
                &collection_info.royalties,
                &ManagedBuffer::new(),
                &attributes,
                &uris,
            );

            nft_output_payments.push(EsdtTokenPayment::new(
                nft_token_id.clone(),
                nft_nonce,
                nft_amount,
            ));
            output_nfts.push((nft_token_id.clone(), nft_nonce, nft_id).into());
        }

        self.send().direct_multi(to, &nft_output_payments);

        output_nfts
    }

    fn claim_common(
        &self,
        claim_allowed_address: ManagedAddress,
        mapper: &mut MapMapper<EgldOrEsdtTokenIdentifier, BigUint>,
    ) -> EgldValuePaymentsVecPair<Self::Api> {
        let caller = self.blockchain().get_caller();
        require!(caller == claim_allowed_address, "Claim not allowed");

        let mut egld_value = BigUint::zero();
        let mut other_payments = ManagedVec::new();
        for (token, amount) in mapper.iter() {
            if token.is_egld() {
                egld_value = amount;
            } else {
                other_payments.push(EsdtTokenPayment::new(token.unwrap_esdt(), 0, amount));
            }
        }

        mapper.clear();

        if egld_value > 0 {
            self.send().direct_egld(&caller, &egld_value);
        }
        if !other_payments.is_empty() {
            self.send().direct_multi(&caller, &other_payments);
        }

        (egld_value, other_payments).into()
    }

    fn add_mint_payment(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        let mut mapper = self.accumulated_mint_payments();
        self.add_common(&mut mapper, token, amount);
    }

    fn add_royalties(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        let mut mapper = self.accumulated_royalties();
        self.add_common(&mut mapper, token, amount);
    }

    fn add_royalties_multiple(&self, payments: &ManagedVec<EsdtTokenPayment<Self::Api>>) {
        let mut mapper = self.accumulated_royalties();
        for p in payments {
            self.add_common(
                &mut mapper,
                EgldOrEsdtTokenIdentifier::esdt(p.token_identifier),
                p.amount,
            );
        }
    }

    fn add_common(
        &self,
        mapper: &mut MapMapper<EgldOrEsdtTokenIdentifier, BigUint>,
        token: EgldOrEsdtTokenIdentifier,
        amount: BigUint,
    ) {
        match mapper.get(&token) {
            Some(mut prev_amount) => {
                prev_amount += amount;
                let _ = mapper.insert(token, prev_amount);
            }
            None => {
                let _ = mapper.insert(token, amount);
            }
        }
    }

    fn get_next_random_id(
        &self,
        collection_id: &CollectionId<Self::Api>,
    ) -> UniqueId {
        let mut id_mapper = self.available_ids(collection_id);
        let last_id_index = id_mapper.len();
        require!(last_id_index > 0, "No more NFTs available for collection");

        let rand_index = self.get_random_usize(VEC_MAPPER_FIRST_ITEM_INDEX, last_id_index + 1);
        let rand_id = id_mapper.swap_remove(rand_index);

        rand_id
    }

    fn verify_nft_id(
        &self,
        collection_id: &CollectionId<Self::Api>,
        nft_id: usize,
    ) -> UniqueId {
        let mut id_mapper = self.available_ids(collection_id);
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
}
