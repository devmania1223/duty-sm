elrond_wasm::imports!();

use nft_collection::structs::EgldValuePaymentsVecPair;

#[elrond_wasm::module]
pub trait PrivateFunctionsModule:
    crate::views::ViewsModule {
    fn store_new_reward_entry(&self) -> usize {
        let new_entry_id = self.last_entry_id().update(|id| {
            *id += 1;
            *id
        });

        new_entry_id
    }

    fn copy_shareholders_to_claim_whitelist(&self, entry_id: usize) {
        let mut new_mapper = self.claim_whitelist_for_entry(entry_id);
        for sh in self.shareholders().iter() {
            new_mapper.insert(sh.address);
        }
    }

    fn add_balance(&self, token: EgldOrEsdtTokenIdentifier, amount: &BigUint) {
        self.balance_for_token(&token).update(|b| {
            *b += amount;
        });
        let _ = self.known_tokens().insert(token);
    }

    fn update_balance_from_results(&self, result: EgldValuePaymentsVecPair<Self::Api>) {
        let (egld_value, other_payments) = result.into_tuple();

        if egld_value > 0 {
            self.add_balance(EgldOrEsdtTokenIdentifier::egld(), &egld_value);
        }
        for p in &other_payments {
            self.add_balance(
                EgldOrEsdtTokenIdentifier::esdt(p.token_identifier),
                &p.amount,
            );
        }
    }    
}
