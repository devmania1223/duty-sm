elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub type EgldValuePaymentsVecPair<M> = MultiValue2<BigUint<M>, PaymentsVec<M>>;
pub type CollectionId<M> = ManagedBuffer<M>;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct CollectionInfo<M: ManagedTypeApi> {
    pub collection_hash: CollectionHash<M>,
    pub token_display_name: ManagedBuffer<M>,
    pub media_type: MediaType<M>,
    pub royalties: BigUint<M>,
    pub mint_period: TimePeriod,
    pub whitelist_expire_timestamp: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct TimePeriod {
    pub start: u64,
    pub end: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct MintPrice<M: ManagedTypeApi> {
    pub token_id: EgldOrEsdtTokenIdentifier<M>,
    pub amount: BigUint<M>,
}

pub const NFT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD
pub const ROYALTIES_MAX: u32 = 10_000; // 100%

pub const MAX_COLLECTION_ID_LEN: usize = 50;
pub static INVALID_COLLECTION_ID_ERR_MSG: &[u8] = b"Invalid Collection ID";

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct TempCallbackTierInfo<M: ManagedTypeApi> {
    pub total_nfts: usize,
    pub mint_price: MintPrice<M>,
}

#[derive(TopEncode, TopDecode)]
pub struct TempCallbackStorageInfo<M: ManagedTypeApi> {
    pub collection_info: CollectionInfo<M>,
    pub tags: Tag<M>,
    pub tier_info: TempCallbackTierInfo<M>
}

pub static ATTRIBUTES_SEPARATOR: &[u8] = b";";

pub static SUPPORTED_MEDIA_TYPES: &[&[u8]] = &[
    b"png",
    b"jpeg",
    b"jpg",
    b"gif",
    b"acc",
    b"flac",
    b"m4a",
    b"mp3",
    b"wav",
    b"mov",
    b"quicktime",
    b"mp4",
    b"webm",
];
pub const MAX_MEDIA_TYPE_LEN: usize = 9;
pub const COLLECTION_HASH_LEN: usize = 46;

pub type Uri<M> = ManagedBuffer<M>;
pub type CollectionHash<M> = ManagedByteArray<M, COLLECTION_HASH_LEN>;
pub type Tag<M> = ManagedBuffer<M>;
pub type MediaType<M> = ManagedBuffer<M>;
pub type GenericAttributes<M> = ManagedBuffer<M>;

pub const NFT_AMOUNT: u32 = 1;


#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct CollectionInfoViewResultType<M: ManagedTypeApi> {
    pub collection_id: CollectionId<M>,
    pub nft_token_id: TokenIdentifier<M>,
    pub collection_info: CollectionInfo<M>,
    pub tier_info: TierInfoEntry<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Debug, PartialEq)]
pub struct TierInfoEntry<M: ManagedTypeApi> {
    pub total_nfts: usize,
    pub available_nfts: usize,
    pub mint_price: MintPrice<M>,
}


pub const VEC_MAPPER_FIRST_ITEM_INDEX: usize = 1;