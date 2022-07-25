elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Debug, PartialEq)]
pub struct AddressPair<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub percent: BigUint<M>,
}

#[elrond_wasm::module]
pub trait CommonStorageModule {
    #[view(getLastClaimEpoch)]
    #[storage_mapper("lastClaimEpoch")]
    fn last_claim_epoch(&self) -> SingleValueMapper<u64>;

    #[view(getShareholders)]
    #[storage_mapper("shareholders")]
    fn shareholders(&self) -> UnorderedSetMapper<AddressPair<Self::Api>>;
}
