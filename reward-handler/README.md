# DutyRewardHandler

## Endpoints

`add_shareholders` - set shareholders  
`remove_shareholders` - remove shareholders  
`claim_rewards` - claim rewards gathered to this smart contract by owner, caller is shareholder   
`create_new_reward_entry` - create new reward entry for distribution of rewards to shareholders  
`claim_nft_collection_payments_and_royalties` - claim nft collection payments and royalties from DutyNftMinter smart contract  

## View functions and Storages

`get_claimable_entry_ids_for_address` - get claimable entry ids for shareholder's address  
`get_claimable_tokens_for_reward_entry` - get claimable tokens info for reward entry id  
`get_token_balances` - get token balances that caller has  
`last_claim_epoch` - get epoch that claimed last  
`shareholders` - return list of shareholders address and percentage  
`last_reward_entry_epoch` - return last epoch for reward entry  
`last_entry_id` - return entry id that created last  
`claim_whitelist_for_entry` - return whitelist for shareholders that claimed certain entry  
`claimable_tokens_for_reward_entry` - claimable tokens of shareholder for reward entry  
`known_tokens` - return tokens identifier list that claimed so far  
`balance_for_token` - return balance of certain token that accumlated  

## Private functions

`store_new_reward_entry` - increase last_entry_id  
`copy_shareholders_to_claim_whitelist` - copy shareholders list to claimable whitelist  
`add_balance` - add token amount to balance_for_token and known_tokens    
`update_balance_from_results` - update balance_for_token and known_tokens from result of calling claim rewards 

