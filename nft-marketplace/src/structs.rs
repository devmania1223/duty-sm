elrond_wasm::imports!();

elrond_wasm::derive_imports!();

pub const PERCENTAGE_TOTAL: u64 = 10_000; // 100%
pub const NFT_AMOUNT: u32 = 1; // Token has to be unique to be considered NFT

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Auction<M: ManagedTypeApi> {
    pub auctioned_tokens: EsdtTokenPayment<M>,
    pub auction_type: AuctionType,

    pub payment_token: EgldOrEsdtTokenIdentifier<M>,
    pub payment_nonce: u64,
    pub min_bid: BigUint<M>,
    pub max_bid: Option<BigUint<M>>,
    pub start_time: u64,
    pub deadline: u64,

    pub original_owner: ManagedAddress<M>,
    pub current_bid: BigUint<M>,
    pub current_winner: ManagedAddress<M>,
    pub marketplace_fee_percentage: BigUint<M>,
    pub creator_royalties_percentage: BigUint<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum AuctionType {
    None,
    BidType,
    FixedType,
}

pub struct BidSplitAmounts<M: ManagedTypeApi> {
    pub creator: BigUint<M>,
    pub marketplace: BigUint<M>,
    pub seller: BigUint<M>,
}

