elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::structs::{Stake};

#[allow(clippy::too_many_arguments)]
#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_create_stake_event(self, stake_id: u64, stake: Stake<Self::Api>){
        self.create_stake_event(
            &stake.staked_tokens.token_identifier,
            stake.staked_tokens.token_nonce,
            stake_id,
            &stake.staked_tokens.amount,
            &stake.owner,
            stake.stake_timestamp
        );
    }

    fn emit_unstake_event(self, stake_id: u64, stake: Stake<Self::Api>) {
        self.unstake_event(
            &stake.staked_tokens.token_identifier,
            stake.staked_tokens.token_nonce,
            stake_id,
            &stake.staked_tokens.amount,
            &stake.owner
        );
    }

    #[event("create_stake_event")]
    fn create_stake_event(
        &self,
        #[indexed] create_stake_id: &TokenIdentifier,
        #[indexed] stake_token_nonce: u64,
        #[indexed] stake_id: u64,
        #[indexed] staked_token_amount: &BigUint,
        #[indexed] stake_creator: &ManagedAddress,
        #[indexed] stake_timestamp: u64
    );

    #[event("unstake_event")]
    fn unstake_event(
        &self,
        #[indexed] create_stake_id: &TokenIdentifier,
        #[indexed] staked_token_nonce: u64,
        #[indexed] stake_id: u64,
        #[indexed] staked_token_amount: &BigUint,
        #[indexed] stake_creator: &ManagedAddress
    );

}