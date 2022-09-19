elrond_wasm::imports!();

use crate::structs::{Stake};

#[elrond_wasm::module]
pub trait ViewsModule {
    #[view(getFullStakeData)]
    fn try_get_stake(&self, stake_id: u64) -> Stake<Self::Api> {
        let stake_mapper = self.stake_by_id(stake_id);
        require!(!stake_mapper.is_empty(), "Stake does not exist");
        stake_mapper.get()
    }

    #[view(getStakedCount)]
    #[storage_mapper("stakedCount")]
    fn staked_count(&self) -> SingleValueMapper<u64>;

    #[view(getLastStakedId)]
    #[storage_mapper("lastStakedId")]
    fn last_staked_id(&self) -> SingleValueMapper<u64>;

    #[view(getLastClaimTimestamp)]
    #[storage_mapper("lastTimestamp")]
    fn last_claim_timestamp((&self) -> SingleValueMapper<u64>);

    #[storage_mapper("stakeById")]
    fn stake_by_id(&self, stake_id: u64) -> SingleValueMapper<Stake<Self::Api>>;
}