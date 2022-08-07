# DutyNftMarketplace

## Endpoints

`set_fee_percentage` - set fee_percentage  
`create_auction` - create auction for bid and buy  
`end_auction` - end auction and distribute tokens after auction end   
`cancel_auction` - cancel auction and return nft to caller  
`bid` - set current winner of auction to caller and refund losing bid if success  
`buy` - set current winner and current bid to caller and payment amount and distrubute tokens if success
`claim_tokens` - send claimable amount of caller to claim_destination  

## View functions and Storages

`try_get_auction` - get auction if it exists  
`last_valid_auction_id` - storage_mapper for auction id that created last  
`fee_percentage` - storage_mapper for percentage of marketplace    
`claimable_amount` - storage_mapper for claimable token amount    
`auction_by_id` - storage_mapper for auction by id  

## Private functions

`is_valid_bid` - check if the bid is valid  
`get_nft_info` - get nft info by blockchain api  
`try_set_fee_percentage` - set fee percentage if it's valid  
`calculate_amount` - calculate fee amount corresponding fee percentage  
`calculate_winning_bid_split` - calculate the winning bid split  
`distribute_tokens_after_auction_end` - distribute tokens to marketplace owner, collection creator, bid winner,...  
`transfer_or_save_payment` - transfer token or update claim amount  

## Events

`create_auction_event` - event that is emitted after creating auction    
`end_auction_event` - event that is emitted after ending auction    
`cancel_auction_event` - event that is emitted after cancelling collecauctiontion    
`bid_event` - event that is emitted after bidding nft    
`buy_event` - event that is emitted after buying nft    