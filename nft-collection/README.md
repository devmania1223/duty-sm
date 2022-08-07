# DutyNftMinter

## Endpoints

`add_user_to_admin_list` - add user to the list of admin who can create collection, manage whitelist, ...  
`remove_user_from_admin_list` - remove user from the list of admin who can create collection, manage whitelist, ...  
`add_collection` - add collection and issue nft(not mint) for this collection   
`set_mint_token` - set mint token of payment before user mints  
`add_to_whitelist` - add user address list to whitelist  
`remove_from_whitelist` - remove user address list to whitelist  
`set_mint_whitelist_expire_timestamp` - set mint whitelist expire timestamp  
`mint_nft` - mint nft to collection  
`giveaway_nfts` - send giveaway nfts  
`claim_nfts` - claim random nfts by amount  
`claim_nfts_by_ids` - claim certain nfts by ids  
`set_royalties_claim_address` - set royalties_claim_address  
`set_mint_payments_claim_address` - set mint_payments_claim_address  
`claim_royalties` - send royalties to royalties_claim_address  
`claim_mint_payments` - send mint fee to mint_payments_claim_address  
`claim_royalties_from_marketplace` - claim royalties from marketplace by calling claim_tokens function in nft marketplace contract  

## View functions and Storages

`get_collection_info_view` - get nft token id, collection info, total_nfts, available_nfts, mint_price for certion collection id  
`get_all_collections_info` - get result of get_collection_info_view  for all collections  
`registered_collection_hashes` - storage_mapper for registered collection hashes  
`registered_collections` - storage_mapper for registered collection ids  
`nft_token` - storage_mapper for nft token of certain collection  
`collection_info` - storage_mapper for collection info(collection_hash, token_display_name, media_type, royalties, mint_period, whitelist_expire_timestamp)  
`price_for_tier` - storage_mapper for mint price info(token_id, amount) 
`tags_for_collection` - storage_mapper for tags of collection  
`mint_whitelist` - storage_mapper for tags of collection  
`royalties_claim_address` - storage_mapper for  royalties claim address
`mint_payments_claim_address` - storage_mapper for  mint fee claim address  
`accumulated_royalties` -  storage_mapper for accumulated royalties    
`accumulated_mint_payments` - storage_mapper for accumulated mint fee  
`admin_whitelist` - storage_mapper for admin whitelist  
`available_ids` - storage_mapper for available nft ids of collection  
`total_nfts` - storage_mapper for total nfts of collection  
`temporary_callback_storage` - storage_mapper that temporarily used in adding collection  

## Private functions

`require_caller_is_admin` - require function that verify if caller is admin  
`build_nft_attributes` - build nft attributes  
`build_attributes_metadata_part` - build nft attributes of metadata part   
`build_nft_main_file_uri` - build uri of nft main file   
`build_nft_json_file_uri` - build uri of nft json file  
`build_collection_json_file_uri` - build uri of collection json file   
`is_supported_media_type` - check if media type is supported    
`_mint_and_send_random_nft` - mint and send nft to the address in param  
`claim_common` - send payment egld and tokens(in param) to the caller  
`add_mint_payment` - add mint fee to accumulated_mint_payments  
`add_royalties` - add royalties to accumulated_royalties  
`add_royalties_multiple` - add multiple royalties to accumulated_royalties    
`add_common` - common function to add mint fee and royalties  
`get_next_random_id` - get random id for newly minted nft  
`verify_nft_id` - verify if nft id is minted already  
`get_random_usize` - used to get next random id 

## Events

`collection_created_event` - event that is emitted after adding collection  
`nft_bought_event` - event that is emitted after minting nft  
`nft_giveaway_event` - event that is emitted after sending giveaway  
`nft_claimed_event` - event that is emitted after claiming nfts  