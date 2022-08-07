# Duty NFT marketplace
## Introduction

NFT marketplace is a contract where you can buy and sell non-fungible tokens for fungible tokens. The contract also supports holding the NFT auctions and making/accepting purchase offers on NFTs.

Duty NFT marketplace has special protocal on Elrond network. 
This article explains the programming interface, data structure, basic functions and explains their purpose.

## Structure

Duty NFT marketplace is consist of three smart contracts; DutyNftMinter, DutyNftMarketplace, DutyRewardHandler smart contract

### 1. DutyNftMinter

DutyNftMinter has role of adding collection, nft minting, giveway nfts and claim nfts
### 2. DutyNftMarketplace

Auction can be created and ended, bid and buy actions are performed at DutyNftMarketplace smart contract
### 3. DutyRewardHandler

DutyRewardHandler gathers rewards(mint fee and royalties) and distribute to shareholders
## Functionality
### NFT auction

The marketplace contract includes the auction. Auction of duty is an open auction at an increasing price, where participants openly bid against each other, with each subsequent bid being greater than the previous one.

### NFT purchase

The user can directly buy nft which is listed with fixed price.
### Royalties

Royalties represent a specific percentage of currency the original creator of the NFT receives after each selling. The marketplace itself also takes its own percentage of royalties for hosting the auction.  

## General Logic

### Step 1. Creating a collection

First, you have to create nft collection, by calling the `addCollection` endpoint of DutyNftMinter smart contract, described below:

```
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
) 
```

`collection_hash` - the hash value at ipfs of directory for nft assets(metadata json file, image file, ...) of the collection  
`collection_id` - identifier for collection in smart contract, this should be generated externally  
`media_type` - the extension of nft image file(png, jpg, ...)  
`royalties` - the percentage of royalty, 10_000 for 100%  
`mint_start_timestamp` - the timestamp when user can start minting  
`mint_end_timestamp` - the timestamp when user should end minting, user can't mint after this timestamp  
`mint_price_token_id` - the token identifier of payment for minting  
`token_display_name` - the name of non-fungible token which user mints  
`token_ticker` - the ticker of non-fungible token which user mints  
`whitelist_expire_timestamp` - the timestamp when check the user is whitelisted  
`tags` - the tags of nft data, to be added in medadata  
`total_nfts` - total count of nfts that can be minted in this collection      
`mint_price_token_amount` - the token amount of payment for minting  


### Step 2. Nft Minting    
  

Now, minters can mint nft, by calling the `mintNft` endpoint of DutyNftMinter smart contract, described below:

```
#[payable("*")]
#[endpoint(mintNft)]
fn mint_nft(
    &self,
    collection_id: CollectionId<Self::Api>,
    opt_nfts_to_buy: OptionalValue<usize>,
)-> PaymentsVec<Self::Api>
```

`collection_id` - collection id to mint nft  
`opt_nfts_to_buy` - count of nfts to mint(optional value, count is to be 1 when omit this param)    
This endpoint returns minted nft informations like token identifier, token nonce and these are used in creating auction and bidding  

You can make giveaway nfts, by calling the `giveawayNfts` endpoint of DutyNftMinter smart contract, described below:  

```
#[endpoint(giveawayNfts)]
fn giveaway_nfts(
    &self,
    collection_id: CollectionId<Self::Api>,
    dest_amount_pairs: MultiValueEncoded<MultiValue2<ManagedAddress, usize>>,
)
```

`collection_id` - collection id to send giveaway nfts  
`dest_amount_pairs` - pairs of address and amount to send    

You can claim nfts by count or ids, by calling the `claimNfts` and `claimNftsByIds` endpoint of DutyNftMinter smart contract, described below:  

```
#[endpoint(claimNfts)]
fn claim_nfts(
    &self,
    collection_id: CollectionId<Self::Api>,
    claim_amount: usize,
)
```

`collection_id` - collection id to claim nfts  
`claim_amount` - amount of nfts to claim    


```
#[endpoint(claimNftsByIds)]
fn claim_nfts_by_ids(
    &self,
    collection_id: CollectionId<Self::Api>,
    nft_ids: MultiValueEncoded<usize>,
)
```

`collection_id` - collection id to claim nfts  
`nft_ids` - list of id to claim    

### Step 3. Starting Auction

First, you have to transfer the NFT to the marketplace, by calling the `createAuction` endpoint of DutyNftMarketplace smart contract, described below:

```
#[payable("*")]
#[endpoint(createAuction)]
#[allow(clippy::too_many_arguments)]
fn create_auction(
    &self,
    min_bid: BigUint,
    max_bid: BigUint,
    deadline: u64,
    accepted_payment_token: EgldOrEsdtTokenIdentifier,
    opt_is_fixed_price: OptionalValue<bool>,
    opt_accepted_payment_token_nonce: OptionalValue<u64>,
    opt_start_time: OptionalValue<u64>,
) -> u64
```

Arguments are about what you would expect for an marketplace: 
`min_bid` - lowest amount someone can bid.  
`max_bid` - maximum bid. If this is reached, auction can be ended before the deadline.  
`deadline` - the deadline for the auction, expressed as a unix timestamp.  
`accepted_payment_token` - The token you wish to receive as payment. For eGLD, input `EGLD`.  
`opt_accepted_payment_token_nonce` - "nonce" (also known as "id") for the ESDT token. For usual ESDTs (not NFTs), this is 0 and can be skipped.  
`opt_start_time` - start timestamp to bid  
This endpoint returns identifer of auction that can be used in bidding and ending auction  
### Step 4. Bidding

To bid on an auctioned token, you call the `bid` endpoint of DutyNftMarketplace smart contract:  

```
#[payable("*")]
#[endpoint]
fn bid(&self, auction_id: u64, nft_type: TokenIdentifier, nft_nonce: u64)
```

`auction_id` - identifier of auction to bid   
`nft_type` - token identifier of nft to bid(created while adding collection)    
`nft_nonce` - nonce of nft(sequence of nft in collection)   

Pretty straight forward, the `nft_type` is the "class" of tokens you want to bid on, and the `nft_nonce` is the specific nft id.  

If the bid is valid (higher than the previous), the previous bid (if any) is cancelled and the payment tokens are sent back to the previous bidder.  

### Step 5. Ending an auction

Once the deadline has passed or the maximum bid has been made, the auction can be ended by calling the `endAuction` endpoint of DutyNftMarketplace smart contract: 

```
#[endpoint(endAuction)]
fn end_auction(&self, auction_id: u64)
```
`auction_id` - identifier of auction to end   

If no bids were made, the NFT is returned to the owner. If bids were made, the NFT is sent to the highest bidder, and the bid is split between the NFT creator, marketplace SC and NFT owner.  


### Step 6. Creating reward entry

While minters mint nft and nft is sold at marketplace, the reward is created and collection owner is creating reward entry by calling the `createNewRewardEntry` endpoint of DutyRewardHandler smart contract: 

```
#[only_owner]
#[endpoint(createNewRewardEntry)]
fn create_new_reward_entry(&self)
```
`auction_id` - identifier of auction to end   



### Step 7. Claiming reward  

Once the reward entry has been created, the shareholders can claim reward by calling the `claimRewards` endpoint of DutyRewardHandler smart contract: 

```
#[endpoint(claimRewards)]
fn claim_rewards(&self, entry_ids: MultiValueEncoded<usize>)
```
`entry_ids` - list of entry id(created by owner to totalize detailed distribution of reward to shareholders)   

