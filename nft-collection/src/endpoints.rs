#![no_std]

elrond_wasm::imports!();

pub mod events;
pub mod nft_marketplace_interactor;
pub mod private_functions;
pub mod structs;
pub mod views;
pub mod storage;

use crate::structs::{CollectionId, CollectionInfo, CollectionHash, TimePeriod, TempCallbackStorageInfo,
    TempCallbackTierInfo, MintPrice, EgldValuePaymentsVecPair, 
    MAX_COLLECTION_ID_LEN, INVALID_COLLECTION_ID_ERR_MSG, NFT_ISSUE_COST, ROYALTIES_MAX, NFT_AMOUNT};

/// @author Josh Brolin
/// @title DutyNftMinter
/// @dev smart contract for collection and nft minting
#[elrond_wasm::contract]
pub trait DutyNftMinter:
 nft_marketplace_interactor::NftMarketplaceInteractorModule
    + private_functions::PrivateFunctionsModule
    + views::ViewsModule
    + events::EventsModule
    + storage::StorageModule
{
    /// Initialize smart contract
    /// @param royalties_claim_address: the address of the account or contract who can claim royalties
    /// @param mint_payments_claim_address: the address of the account or contract who can claim mint fee
    /// @dev constructor of smart contract
    #[init]
    fn init(
        &self,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
    ) {
        self.royalties_claim_address().set(&royalties_claim_address);
        self.mint_payments_claim_address()
            .set(&mint_payments_claim_address);
    }

    /// Add user to admin list
    /// @param address: the address to be added to admin list
    /// @dev add user to the list of admin who can create collection, manage whitelist, ...
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by owner
    #[only_owner]
    #[endpoint(addUserToAdminList)]
    fn add_user_to_admin_list(&self, address: ManagedAddress) {
        self.admin_whitelist().add(&address);
    }

    /// Remove user from admin list
    /// @param address: the address to be added to admin list
    /// @dev remove user from the list of admin who can create collection, manage whitelist, ...
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by owner
    #[only_owner]
    #[endpoint(removeUserFromAdminList)]
    fn remove_user_from_admin_list(&self, address: ManagedAddress) {
        self.admin_whitelist().remove(&address);
    }

    /// Add collection
    /// @param collection_hash: the hash value at ipfs of directory for nft assets(metadata json file, image file, ...) of the collection
    /// @param collection_id: identifier for collection in smart contract, this should be generated externally
    /// @param media_type: the extension of nft image file(png, jpg, ...)
    /// @param royalties: the percentage of royalty, 10_000 for 100%
    /// @param mint_start_timestamp: the timestamp when user can start minting
    /// @param mint_end_timestamp: the timestamp when user should end minting, user can't mint after this timestamp
    /// @param mint_price_token_id: the token identifier of payment for minting
    /// @param token_display_name: the name of non-fungible token which user mints
    /// @param token_ticker: the ticker of non-fungible token which user mints
    /// @param whitelist_expire_timestamp: the timestamp when check the user is whitelisted
     ///             if current timestamp is before this timestamp, check of whitelist is enabled, otherwise disabled
    /// @param tags: the tags of nft data, to be added in medadata
    /// @param total_nfts: total count of nfts that can be minted in this collection
    /// @param mint_price_token_amount: the token amount of payment for minting
    /// @dev add collection and issue nft(not mint) for this collection, 
    ///          emit the event of collection creation which exposes collection id and token id of nft
    ///      ✔️payable  non-payable
    ///      requires: - only can be called by admin
    ///                - the length of collection_id is shorter than MAX_COLLECTION_ID_LEN(50)
    ///                - issue costs NFT_ISSUE_COST(50_000_000_000_000_000 for 0.05 EGLD)
    ///                - media_type should be supported media type(png, jpeg, jpg, gif, acc, flac, m4a, mp3, wav, mov, quicktime, mp4, webm)
    ///                - royalties should be less than ROYALTIES_MAX(10_000 for 100%)
    ///                - mint_price_token_id should be valid
    ///                - collection_hash should be unique
    ///                - collection_id should be unique
    ///                - mint_start_timestamp should be less than mint_end_timestamp
    ///                - total_nfts should be greater than zero
    ///                - mint_price_token_amount should be greater than zero
    #[payable("EGLD")]
    #[endpoint(addCollection)]
    fn add_collection(
        &self,
        collection_hash: CollectionHash<Self::Api>,
        collection_id: CollectionId<Self::Api>,
        media_type: ManagedBuffer,
        royalties: BigUint,
        mint_start_timestamp: u64,
        mint_end_timestamp: u64,
        mint_price_token_id: EgldOrEsdtTokenIdentifier,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        whitelist_expire_timestamp: u64,
        tags: ManagedBuffer,
        total_nfts: usize,
        mint_price_token_amount: BigUint,
    ) {
        self.require_caller_is_admin();

        let id_len = collection_id.len();
        require!(
            id_len > 0 && id_len <= MAX_COLLECTION_ID_LEN,
            INVALID_COLLECTION_ID_ERR_MSG
        );

        let payment_amount = self.call_value().egld_value();
        require!(
            payment_amount == NFT_ISSUE_COST,
            "Invalid payment amount. Issue costs exactly 0.05 EGLD"
        );

        require!(
            self.is_supported_media_type(&media_type),
            "Invalid media type"
        );
        require!(royalties <= ROYALTIES_MAX, "Royalties cannot be over 100%");
        require!(mint_price_token_id.is_valid(), "Invalid price token");

        let is_new_collection = self
            .registered_collection_hashes()
            .insert(collection_hash.clone());
        require!(is_new_collection, "Collection hash already exists");

        let is_new_collection = self.registered_collections().insert(collection_id.clone());
        require!(is_new_collection, "Collection already exists");

        require!(
            mint_start_timestamp < mint_end_timestamp,
            "Invalid timestamps"
        );
        require!(
            total_nfts > 0,
            "Invalid total nfts"
        );
        require!(
            mint_price_token_amount > 0,
            "Invalid mint price token amount"
        );      

        let collection_info = CollectionInfo {
            collection_hash: collection_hash.clone(),
            token_display_name: token_display_name.clone(),
            media_type,
            royalties,
            mint_period: TimePeriod {
                start: mint_start_timestamp,
                end: mint_end_timestamp,
            },
            whitelist_expire_timestamp,
        };

        self.mint_whitelist(&collection_id).insert(self.blockchain().get_caller());

        self.temporary_callback_storage(&collection_id)
            .set(&TempCallbackStorageInfo {
                collection_info,
                tags,
                tier_info: TempCallbackTierInfo {
                    total_nfts: total_nfts,
                    mint_price: MintPrice {
                        token_id: mint_price_token_id.clone(),
                        amount: mint_price_token_amount,
                    },
                },
            });

        self.nft_token(&collection_id).issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount,
            token_display_name,
            token_ticker,
            0,
            Some(self.callbacks().issue_callback(collection_hash, collection_id)),
        );
    }

    #[callback]
    fn issue_callback(
        &self,
        collection_hash: CollectionHash<Self::Api>,
        collection_id: CollectionId<Self::Api>,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let cb_info: TempCallbackStorageInfo<Self::Api> =
                    self.temporary_callback_storage(&collection_id).get();

                self.nft_token(&collection_id).set_token_id(&token_id);
                self.collection_info(&collection_id).set(&cb_info.collection_info);                

                self.available_ids(&collection_id)
                        .set_initial_len(cb_info.tier_info.total_nfts);
                self.total_nfts(&collection_id)
                    .set(cb_info.tier_info.total_nfts);

                self.price_for_tier(&collection_id)
                    .set(&cb_info.tier_info.mint_price);

                if !cb_info.tags.is_empty() {
                    self.tags_for_collection(&collection_id).set(&cb_info.tags);
                }

                self.collection_created_event(&collection_id, &token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                let _ = self.registered_collections().swap_remove(&collection_id);
                let _ = self
                    .registered_collection_hashes()
                    .swap_remove(&collection_hash);
            }
        }

        self.temporary_callback_storage(&collection_id).clear();
    }

    /// Set mint token identifier and amount for mint fee
    /// @param collection_id: collection id to set mint token
    /// @param mint_price_token_id: token identifier that can be used payment for mint fee
    /// @param mint_price_token_amount: the token amount of mint fee
    /// @dev set mint token of payment before user mints
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by admin
    #[endpoint(setMintToken)]
    fn set_mint_token(
        &self,
        collection_id: CollectionId<Self::Api>,
        mint_price_token_id: EgldOrEsdtTokenIdentifier,
        mint_price_token_amount: BigUint,
    ) {
        self.require_caller_is_admin();

        self.price_for_tier(&collection_id)
                    .set(MintPrice {
                        token_id: mint_price_token_id.clone(),
                        amount: mint_price_token_amount,
                    });
    }    

    /// Set mint period for the collection
    /// @param collection_id: collection id to set mint period
    /// @param mint_start_timestamp: the timestamp when user can start minting
    /// @param mint_end_timestamp: the timestamp when user should end minting, user can't mint after this timestamp
    /// @dev set mint period before user mints
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by admin
    #[endpoint(setMintPeriod)]
    fn set_mint_period(
        &self,
        collection_id: CollectionId<Self::Api>,
        mint_start_timestamp: u64,
        mint_end_timestamp: u64,
    ) {
        self.require_caller_is_admin();

        self.collection_info(&collection_id)
            .update(|info| info.mint_period = TimePeriod {
                start: mint_start_timestamp,
                end: mint_end_timestamp,
            });
    }

    /// Add users to whitelist
    /// @param collection_id: collection id to add users
    /// @param users: list of user addresses to add to whitelist
    /// @dev add user address list to whitelist 
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by admin
    #[endpoint(addToWhitelist)]
    fn add_to_whitelist(
        &self,
        collection_id: CollectionId<Self::Api>,
        users: MultiValueEncoded<ManagedAddress>,
    ) {
        self.require_caller_is_admin();

        let mut mapper = self.mint_whitelist(&collection_id);
        for user in users {
            let _ = mapper.insert(user);
        }
    }

    /// Remove users from whitelist
    /// @param collection_id: collection id to remove users
    /// @param users: list of user addresses to remove from whitelist
    /// @dev remove user address list to whitelist 
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by admin
    #[endpoint(removeFromWhitelist)]
    fn remove_from_whitelist(
        &self,
        collection_id: CollectionId<Self::Api>,
        users: MultiValueEncoded<ManagedAddress>,
    ) {
        self.require_caller_is_admin();

        let mut mapper = self.mint_whitelist(&collection_id);
        for user in users {
            let _ = mapper.swap_remove(&user);
        }
    }

    /// Set whitelist expiretimestamp for minting
    /// @param collection_id: collection id to set whitelist expiretimestamp
    /// @param timestamp: the timestamp when check the user is whitelisted
    ///             if current timestamp is before this timestamp, check of whitelist is enabled, otherwise disabled
    /// @dev set mint whitelist expire timestamp
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by admin
    #[endpoint(setMintWhitelistExpireTimestamp)]
    fn set_mint_whitelist_expire_timestamp(&self, collection_id: CollectionId<Self::Api>, timestamp: u64) {
        self.require_caller_is_admin();

        self.collection_info(&collection_id)
            .update(|info| info.whitelist_expire_timestamp = timestamp);
    }

    /// Mint nfts
    /// @param collection_id: collection id to mint nft
    /// @param opt_nfts_to_buy: count of nfts to mint(optional value, count is to be 1 when omit this param)
    /// @dev mint nft to collection
    ///      ✔️payable  non-payable
    ///      requires: - collection_id should be registered collection_id(it's registered while adding collection)
    ///                - token identifier of call value should be same as token identifier of the collection(registered while adding collection)
    ///                - token amount of call value should be same as the multiplied value of opt_nfts_to_buy and payment token amount(registered while adding collection)
    ///                - current timestamp should be greater than mint_start_timestamp(registered while adding collection)
    ///                - current timestamp should be less than mint_end_timestamp(registered while adding collection)
    ///                - caller should be whitelisted if current timestamp is less than whitelist_expire_timestamp(registered while adding collection)
    #[payable("*")]
    #[endpoint(mintNft)]
    fn mint_nft(
        &self,
        collection_id: CollectionId<Self::Api>,
        opt_nfts_to_buy: OptionalValue<usize>,
    ) -> MultiValueEncoded<MultiValue3<TokenIdentifier, u64, usize>>{
        require!(
            self.registered_collections().contains(&collection_id),
            INVALID_COLLECTION_ID_ERR_MSG
        );

        let nfts_to_buy = match opt_nfts_to_buy {
            OptionalValue::Some(val) => {
                if val == 0 {
                    return MultiValueEncoded::new();
                }
                val
            }
            OptionalValue::None => NFT_AMOUNT as usize,
        };

        let price_for_tier = self.price_for_tier(&collection_id).get();
        let payment = self.call_value().egld_or_single_esdt();
        let total_required_amount = &price_for_tier.amount * (nfts_to_buy as u32);
        require!(
            payment.token_identifier == price_for_tier.token_id
                && payment.amount == total_required_amount,
            "Invalid payment"
        );

        let collection_info: CollectionInfo<Self::Api> = self.collection_info(&collection_id).get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            current_timestamp >= collection_info.mint_period.start,
            "May not mint yet"
        );
        require!(
            current_timestamp < collection_info.mint_period.end,
            "May not mint after deadline"
        );

        let caller = self.blockchain().get_caller();
        if current_timestamp < collection_info.whitelist_expire_timestamp {
            require!(
                self.mint_whitelist(&collection_id).contains(&caller),
                "Not in whitelist"
            );
        }

        self.add_mint_payment(payment.token_identifier, payment.amount);

        let output_payments =
            self._mint_and_send_random_nft(&caller, &collection_id, &collection_info, nfts_to_buy);

        self.nft_bought_event(&caller, &collection_id, nfts_to_buy);

        output_payments
    }

    /// Send giveaway nfts
    /// @param collection_id: collection id to send giveaway nfts
    /// @param dest_amount_pairs: pairs of address and amount to send
    /// @dev send giveaway nfts
    ///      payable  ✔️non-payable    
    ///      requires: - only can be called by admin
    ///                - collection_id should be registered collection_id(it's registered while adding collection)
    #[endpoint(giveawayNfts)]
    fn giveaway_nfts(
        &self,
        collection_id: CollectionId<Self::Api>,
        dest_amount_pairs: MultiValueEncoded<MultiValue2<ManagedAddress, usize>>,
    ) -> MultiValueEncoded<MultiValue4<TokenIdentifier, u64, usize, ManagedAddress>>{
        self.require_caller_is_admin();

        require!(
            self.registered_collections().contains(&collection_id),
            INVALID_COLLECTION_ID_ERR_MSG
        );        

        let collection_info = self.collection_info(&collection_id).get();
        let mut total = 0;
        let mut output_payments = MultiValueEncoded::new();
        for pair in dest_amount_pairs {
            let (dest_address, nfts_to_send) = pair.into_tuple();
            if nfts_to_send > 0 {
                let outputs = self._mint_and_send_random_nft(
                    &dest_address.clone(),
                    &collection_id,
                    &collection_info,
                    nfts_to_send,
                );
                for item in outputs {
                    let (nft_token_id, nft_nonce, nft_id) = item.into_tuple();
                    output_payments.push((nft_token_id, nft_nonce, nft_id, dest_address.clone()).into())
                }
                total += nfts_to_send;
            }
        }

        self.nft_giveaway_event(&collection_id, total);

        output_payments
    }

    /// Claim nfts by amount
    /// @param collection_id: collection id to claim nfts
    /// @param claim_amount: amount of nfts to claim
    /// @dev claim random nfts by amount
    ///      payable  ✔️non-payable    
    ///      requires: - only can be called by admin
    ///                - collection_id should be registered collection_id(it's registered while adding collection)
    ///                - claim_amount should be greater than 0
    #[endpoint(claimNfts)]
    fn claim_nfts(
        &self,
        collection_id: CollectionId<Self::Api>,
        claim_amount: usize,
    ) -> MultiValueEncoded<MultiValue3<TokenIdentifier, u64, usize>>{
        self.require_caller_is_admin();

        require!(
            self.registered_collections().contains(&collection_id),
            INVALID_COLLECTION_ID_ERR_MSG
        );        

        require!(
            claim_amount > 0,
            "Claim amount must be greater than 0"
        );

        let collection_info = self.collection_info(&collection_id).get();
        let output_payments = self._mint_and_send_random_nft(
            &self.blockchain().get_caller(),
            &collection_id,
            &collection_info,
            claim_amount,
        );

        self.nft_claimed_event(&collection_id, claim_amount);

        output_payments
    }

    /// Claim nfts by ids
    /// @param collection_id: collection id to claim nfts
    /// @param nft_ids: list of id to claim
    /// @dev claim certain nfts by ids 
    ///      payable  ✔️non-payable    
    ///      requires: - only can be called by admin
    ///                - collection_id should be registered collection_id(it's registered while adding collection)
    ///                - collection should have enough nfts to claim
    #[endpoint(claimNftsByIds)]
    fn claim_nfts_by_ids(
        &self,
        collection_id: CollectionId<Self::Api>,
        nft_ids: MultiValueEncoded<usize>,
    ) {
        self.require_caller_is_admin();

        require!(
            self.registered_collections().contains(&collection_id),
            INVALID_COLLECTION_ID_ERR_MSG
        );        

        let collection_info = self.collection_info(&collection_id).get();

        let nft_token_id = self.nft_token(&collection_id).get_token_id();
        let mut nft_output_payments = ManagedVec::new();
        let mut nfts_count = 0;
        
        for nft_id in nft_ids {
            let _ = self.verify_nft_id(&collection_id, nft_id);
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
                self.build_nft_attributes(&collection_info.collection_hash, &collection_id, nft_id);
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
            nfts_count += 1;
        }

        require!(
            nfts_count <= self.available_ids(&collection_id).len(),
            "Not enough NFTs available"
        );

        self.send().direct_multi(&self.blockchain().get_caller(), &nft_output_payments);

        self.nft_claimed_event(&collection_id, nfts_count);
    }

    /// Set the address for claim royalties
    /// @param new_address: the address of the account or contract who can claim royalties
    /// @dev set royalties_claim_address 
    ///      payable  ✔️non-payable    
    ///      requires: - only can be called by admin
    #[endpoint(setRoyaltiesClaimAddress)]
    fn set_royalties_claim_address(&self, new_address: ManagedAddress) {
        self.require_caller_is_admin();
        self.royalties_claim_address().set(&new_address);
    }

    /// Set the address for claim mint fee
    /// @param new_address: the address of the account or contract who can claim mint fee
    /// @dev set mint_payments_claim_address 
    ///      payable  ✔️non-payable    
    ///      requires: - only can be called by admin
    #[endpoint(setMintPaymentsClaimAddress)]
    fn set_mint_payments_claim_address(&self, new_address: ManagedAddress) {
        self.require_caller_is_admin();
        self.mint_payments_claim_address().set(&new_address);
    }

    /// Claim royalties
    /// @dev send royalties to royalties_claim_address
    ///      payable  ✔️non-payable    
    #[endpoint(claimRoyalties)]
    fn claim_royalties(&self) -> EgldValuePaymentsVecPair<Self::Api> {
        let royalties_claim_address = self.royalties_claim_address().get();
        let mut mapper = self.accumulated_royalties();

        self.claim_common(royalties_claim_address, &mut mapper)
    }

    /// Claim mint fee
    /// @dev send mint fee to mint_payments_claim_address
    ///      payable  ✔️non-payable    
    #[endpoint(claimMintPayments)]
    fn claim_mint_payments(&self) -> EgldValuePaymentsVecPair<Self::Api> {
        let mint_payments_claim_address = self.mint_payments_claim_address().get();
        let mut mapper = self.accumulated_mint_payments();

        self.claim_common(mint_payments_claim_address, &mut mapper)
    }
}
