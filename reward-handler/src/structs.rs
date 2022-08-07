elrond_wasm::imports!();

elrond_wasm::derive_imports!();

pub const FIRST_ENTRY_ID: usize = 1;
pub const PERCENTAGE_TOTAL: u64 = 10_000; // 100%

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Debug, PartialEq)]
pub struct AddressPair<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub percent: BigUint<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct RewardEntry<M: ManagedTypeApi> {
    pub egld_amount: BigUint<M>,
    pub esdt_payments: nft_collection::structs::PaymentsVec<M>,
}
