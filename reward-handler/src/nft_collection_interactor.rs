elrond_wasm::imports!();

use nft_collection::{structs::EgldValuePaymentsVecPair};

/// @author Josh Brolin
/// @title DutyNftMinterInteractorModule
/// @dev module for interacting DutyNftMinter smart contract
#[elrond_wasm::module]
pub trait DutyNftMinterInteractorModule:
    crate::views::ViewsModule
    + crate::private_functions::PrivateFunctionsModule
{
    /// Claim mint fee and royalties
    /// @dev claim nft collection payments and royalties from DutyNftMinter smart contract
    ///      the owner should claim royalties at DutyNftMinter smart contract before this action
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by owner
    ///                - current epoch should be greater than last_claim_epoch
    #[only_owner]
    #[endpoint(claimDutyNftMinterPaymentsAndRoyalties)]
    fn claim_nft_collection_payments_and_royalties(&self) {
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claim_epoch = self.last_claim_epoch().get();
        require!(
            current_epoch > last_claim_epoch,
            "Already claimed this epoch"
        );

        self.last_claim_epoch().set(&current_epoch);

        let sc_address = self.nft_collection_sc_address().get();

        let royalties_result = self.call_claim_royalties(sc_address.clone());
        self.update_balance_from_results(royalties_result);

        let mint_payments_result = self.call_claim_mint_payments(sc_address);
        self.update_balance_from_results(mint_payments_result);
    }

    fn call_claim_royalties(
        &self,
        sc_address: ManagedAddress,
    ) -> EgldValuePaymentsVecPair<Self::Api> {
        self.nft_collection_proxy(sc_address)
            .claim_royalties()
            .execute_on_dest_context()
    }

    fn call_claim_mint_payments(
        &self,
        sc_address: ManagedAddress,
    ) -> EgldValuePaymentsVecPair<Self::Api> {
        self.nft_collection_proxy(sc_address)
            .claim_mint_payments()
            .execute_on_dest_context()
    }

    #[proxy]
    fn nft_collection_proxy(&self, sc_address: ManagedAddress) -> nft_collection::Proxy<Self::Api>;

    /// @notice view function for nft collection smart contract address
    #[view(getDutyNftMinterScAddress)]
    #[storage_mapper("nftMinterScAddress")]
    fn nft_collection_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}
