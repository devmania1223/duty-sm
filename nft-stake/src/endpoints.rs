#![no_std]

elrond_wasm::imports!();
use crate::structs::{Stake, DAY, PERHOUR_REWARD};

pub mod events;
pub mod private_functions;
pub mod structs;
pub mod views;

/// @author Josh Brolin
/// @title DutyNFTStake
/// @dev smart contract for nft stake

#[elrond_wasm::contract]
pub trait DutyNftStake:
    views::ViewsModule
    + events::EventsModule
    + crate:: private_functions::PrivateFunctionsModule
{
    /// Initialize smart contract
    /// @dev constructor of smart contract
    #[init]
    fn init(&self) {}

    /// Create NFT Stake  
    /// @dev stake for bid and buy
    ///          emit the event of stake creation, 
    /// @return stake_id(id of created stake)
    ///      ✔️payable  non-payable
    ///      requires: - nft_amount(call_value) should be one
    ///                - nft_nonce(call_value) should be greater than zero
    ///                - current timestamp should be less than deadline 
    #[payable("*")]
    #[endpoint(stakeNft)]
    #[allow(clippy::too_many_arguments)]
    fn nft_stake(
        &self
    ) -> u64 {
        let (nft_type, nft_nonce, nft_amount) = self.call_value().single_esdt().into_tuple();

        let current_time = self.blockchain().get_block_timestamp();

        let stake = Stake {
            staked_tokens: EsdtTokenPayment::new(nft_type, nft_nonce, nft_amount),
            owner: self.blockchain().get_caller(),
            stake_timestamp: current_time
        };

        let update_staked_count = self.staked_count().get() + 1;
        self.staked_count().set(&update_staked_count);

        let stake_id = self.last_staked_id().get() + 1;
        self.last_staked_id().set(stake_id);

        self.stake_by_id(stake_id).set(&stake);

        self.emit_create_stake_event(stake_id, stake);

        stake_id
    }

    /// NFT Unstake
    /// @param stake_id: id of stake_id to unstake
    /// @dev unstake and return nft to caller
    ///          emit the event of unstaked
    ///      payable  ✔️non-payable
    ///      requires: - caller should be original owner of stake
    ///                - can not cancel if current id is higher than zero or stake type is fixed type
    #[endpoint(unstakeNft)]
    fn nft_unstakte(
        &self, stake_id: u64
    ) {
        let stake = self.try_get_stake(stake_id);
        let caller = self.blockchain().get_caller();

        require!(
            stake.owner == caller,
            "Only the owner can unstake."
        );

        self.stake_by_id(stake_id).clear();

        let update_staked_count = self.staked_count().get() - 1;
        self.staked_count().set(&update_staked_count);

        let nft_type = &stake.staked_tokens.token_identifier;
        let nft_nonce = stake.staked_tokens.token_nonce;
        let nft_amount = &stake.staked_tokens.amount;
        self.transfer_or_save_payment(
            &caller,
            &EgldOrEsdtTokenIdentifier::esdt(nft_type.clone()),
            nft_nonce,
            nft_amount,
        );

        self.emit_unstake_event(stake_id, stake)
    }

    /// Reward
    /// @param claim_destination: address to receive reward 
    #[endpoint(claimNft)]
    fn nft_claim(
        &self,
        claim_destination: ManagedAddress,
    ){
        let current_time = self.blockchain().get_block_timestamp();
        let last_claim_timestamp = self.last_claim_timestamp().get();
        let staked_nft_count = self.staked_count().get();
 
        require!(
            current_time - last_claim_timestamp < DAY,
            "Can't claim now."
        );

        let reward_amounnt = (current_time - last_claim_timestamp) / 3600000 * staked_nft_count * PERHOUR_REWARD;
        let caller = &self.blockchain().get_caller();

        self.send().direct_esdt(caller, token_nonce: u64, reward_amounnt)
    }
}