elrond_wasm::imports!();
elrond_wasm::derive_imports!(); 

pub const PERHOUR_REWARD: u64 = 0.1; 
pub const DAY: u64 = 86400000

#[derive(TopEncode, TopDecode, TypeAbi)] 
pub struct Stake<M: ManagedTypeApi> {
    pub staked_tokens: EsdtTokenPayment<M>,
    pub owner: ManagedAddress<M>,
    pub stake_timestamp: u64
}
 