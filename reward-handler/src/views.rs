elrond_wasm::imports!();

use crate::structs::{RewardEntry,AddressPair, FIRST_ENTRY_ID};

#[elrond_wasm::module]
pub trait ViewsModule {
    #[view(getClaimableEntryIdsForAddress)]
    fn get_claimable_entry_ids_for_address(
        &self,
        address: ManagedAddress,
        nr_entries_to_look_back: usize,
    ) -> MultiValueEncoded<usize> {
        let mut result = MultiValueEncoded::new();
        let last_id = self.last_entry_id().get();
        if last_id == 0 {
            return result;
        }

        let first_id = if nr_entries_to_look_back >= last_id {
            FIRST_ENTRY_ID
        } else {
            last_id - nr_entries_to_look_back
        };

        for id in first_id..=last_id {
            if self.claim_whitelist_for_entry(id).contains(&address) {
                result.push(id);
            }
        }

        result
    }

    #[view(claimableTokensForRewardEntry)]
    fn get_claimable_tokens_for_reward_entry(
        &self,
        entry_id: usize,
        shareholder: ManagedAddress,
    ) -> MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, BigUint>> {
        let mut result = MultiValueEncoded::new();
        let reward_entry: RewardEntry<Self::Api> =
            self.claimable_tokens_for_reward_entry(entry_id, shareholder).get();

        if reward_entry.egld_amount > 0 {
            result.push((EgldOrEsdtTokenIdentifier::egld(), reward_entry.egld_amount).into());
        }
        for p in &reward_entry.esdt_payments {
            result.push(
                (
                    EgldOrEsdtTokenIdentifier::esdt(p.token_identifier),
                    p.amount,
                )
                    .into(),
            );
        }

        result
    }

    #[view(getTokenBalances)]
    fn get_token_balances(
        &self,
    ) -> MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, BigUint>> {
        let mut balances = MultiValueEncoded::new();

        for token_id in self.known_tokens().iter() {
            let balance_for_token = self.balance_for_token(&token_id).get();
            if balance_for_token > 0 {
                balances.push((token_id, balance_for_token).into());
            }
        }

        balances
    }

    #[view(getLastClaimEpoch)]
    #[storage_mapper("lastClaimEpoch")]
    fn last_claim_epoch(&self) -> SingleValueMapper<u64>;

    #[view(getShareholders)]
    #[storage_mapper("shareholders")]
    fn shareholders(&self) -> UnorderedSetMapper<AddressPair<Self::Api>>;

    #[view(getLastRewardEntryEpoch)]
    #[storage_mapper("lastRewardEntryEpoch")]
    fn last_reward_entry_epoch(&self) -> SingleValueMapper<u64>;

    #[view(getLastEntryId)]
    #[storage_mapper("lastEntryId")]
    fn last_entry_id(&self) -> SingleValueMapper<usize>;

    #[view(getClaimWhitelistForEntry)]
    #[storage_mapper("claimWhitelistForEntry")]
    fn claim_whitelist_for_entry(&self, entry_id: usize) -> UnorderedSetMapper<ManagedAddress>;  

    #[storage_mapper("claimableTokensForRewardEntry")]
    fn claimable_tokens_for_reward_entry(
        &self,
        entry_id: usize,
        shareholder: ManagedAddress,
    ) -> SingleValueMapper<RewardEntry<Self::Api>>;

    #[storage_mapper("knownTokens")]
    fn known_tokens(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("balanceForToken")]
    fn balance_for_token(&self, token_id: &EgldOrEsdtTokenIdentifier)
        -> SingleValueMapper<BigUint>;
}
