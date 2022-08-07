#![no_std]

elrond_wasm::imports!();

pub mod nft_collection_interactor;
pub mod private_functions;
pub mod views;
pub mod structs;

use crate::structs::{RewardEntry, AddressPair, PERCENTAGE_TOTAL};
use nft_collection::structs::PaymentsVec;

/// @author Josh Brolin
/// @title DutyRewardHandler
/// @dev smart contract for reward handler
#[elrond_wasm::contract]
pub trait DutyRewardHandler:
    nft_collection_interactor::DutyNftMinterInteractorModule
    + views::ViewsModule
    + crate::private_functions::PrivateFunctionsModule
{
    /// Initialize smart contract
    /// @param nft_collection_sc_address: smart contract address of nft collection
    /// @dev constructor of smart contract, set shareholder address and percentage to caller and PERCENTAGE_TOTAL(10_000 for 100%)
    #[init]
    fn init(
        &self,
        nft_collection_sc_address: ManagedAddress,
    ) {
        require!(
            self.blockchain().is_smart_contract(&nft_collection_sc_address),
            "Invalid NFT Minter SC address"
        );

        self.nft_collection_sc_address().set(&nft_collection_sc_address);

        let mut sh_addresses = MultiValueEncoded::new();
        sh_addresses.push((self.blockchain().get_caller(), BigUint::from(PERCENTAGE_TOTAL)).into());

        self.add_shareholders(sh_addresses);
    }

    /// Add shareholders
    /// @param shareholders: list of shareholder address and percentage
    /// @dev set shareholders
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by owner
    ///                - sum of percentage should be PERCENTAGE_TOTAL(10_000 for 100%)
    #[only_owner]
    #[endpoint(addShareholders)]
    fn add_shareholders(&self, shareholders: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>) {
        let mut total_percent: BigUint = BigUint::zero();
        for sh in shareholders {
            let (address, percent) = sh.into_tuple();
            self.shareholders().insert(AddressPair{
                address: address, 
                percent: percent.clone(),
            });
            total_percent = &total_percent + &percent;
        }
        require!(
            total_percent == BigUint::from(PERCENTAGE_TOTAL),
            "The sum of percent must be 100%"
        );        
    }

    /// Remove shareholders
    /// @param shareholders: list of shareholder address and percentage
    /// @dev remove shareholders
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by owner
    #[only_owner]
    #[endpoint(removeShareholders)]
    fn remove_shareholders(&self) {
        for shareholder in self.shareholders().iter() {
            let _ = self.shareholders().swap_remove(&shareholder);
        }
    }

    /// Claim rewards
    /// @param entry_ids: list of entry id(created by owner to totalize detailed distribution of reward to shareholders)
    /// @dev claim rewards gathered to this smart contract by owner, caller is shareholder
    ///      payable  ✔️non-payable
    ///      requires: - caller already claimed
    #[endpoint(claimRewards)]
    fn claim_rewards(&self, entry_ids: MultiValueEncoded<usize>) {
        let caller = self.blockchain().get_caller();
        for entry_id in entry_ids {
            let mut whitelist_mapper = self.claim_whitelist_for_entry(entry_id);
            if !whitelist_mapper.contains(&caller.clone()) {
                continue;
            }

            let rewards_entry_mapper = self.claimable_tokens_for_reward_entry(entry_id, caller.clone());
            if rewards_entry_mapper.is_empty() {
                continue;
            }

            let reward_entry: RewardEntry<Self::Api> = rewards_entry_mapper.get();

            let _ = whitelist_mapper.swap_remove(&caller);

            rewards_entry_mapper.clear();

            if reward_entry.egld_amount > 0 {
                self.send().direct_egld(&caller, &reward_entry.egld_amount);
            }
            if !reward_entry.esdt_payments.is_empty() {
                self.send()
                    .direct_multi(&caller, &reward_entry.esdt_payments);
            }
        }
    }

    /// Create new reward entry
    /// @dev create new reward entry for distribution of rewards to shareholders
    ///      reward should be claimed before creation of reward entry
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by owner
    ///                - current epoch should be same as last_claim_epoch
    ///                - entry can not be created for one epoch
    ///                - shareholders should be registered
    #[only_owner]
    #[endpoint(createNewRewardEntry)]
    fn create_new_reward_entry(&self) {
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claim_epoch = self.last_claim_epoch().get();
        require!(
            current_epoch == last_claim_epoch,
            "Must claim rewards for this epoch first"
        );

        self.last_reward_entry_epoch()
            .update(|last_reward_entry_epoch| {
                require!(
                    *last_reward_entry_epoch != current_epoch,
                    "Already created reward entry for this epoch"
                );

                *last_reward_entry_epoch = current_epoch;
            });

        let nr_shareholders = self.shareholders().len() as u32;
        require!(nr_shareholders > 0, "No shareholders");


        let entry_id = self.store_new_reward_entry();
        self.copy_shareholders_to_claim_whitelist(entry_id);

        let total_percent: u32 = 100;

        for shareholder in self.shareholders().iter() {

            let mut esdt_payments = PaymentsVec::new();
            let mut egld_amount = BigUint::zero();

            for token_id in self.known_tokens().iter() {   
                
                let balance_mapper = self.balance_for_token(&token_id);
                let balance = balance_mapper.get();             
    
                let amount_per_holder = &balance * &shareholder.percent / BigUint::from(total_percent);
    
                if token_id.is_egld() {
                    egld_amount = amount_per_holder;
                } else {
                    esdt_payments.push(EsdtTokenPayment::new(
                        token_id.unwrap_esdt(),
                        0,
                        amount_per_holder,
                    ));
                }
    
    
            }
    
            if egld_amount > 0 || !esdt_payments.is_empty() {
    
                self.claimable_tokens_for_reward_entry(entry_id, shareholder.address)
                .set(&RewardEntry {
                    egld_amount,
                    esdt_payments,
                });
    
            }
        }            
    }
}
