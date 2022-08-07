elrond_wasm::imports!();

use crate::structs::EgldValuePaymentsVecPair;

pub mod nft_marketplace_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait NftMarketplaceProxy {
        #[endpoint(claimTokens)]
        fn claim_tokens(
            &self,
            claim_destination: ManagedAddress,
            token_nonce_pairs: MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, u64>>,
        ) -> super::EgldValuePaymentsVecPair<Self::Api>;
    }
}

/// @author Josh Brolin
/// @title NftMarketplaceInteractorModule
/// @dev module for interacting with marketplace to claim royalties
#[elrond_wasm::module]
pub trait NftMarketplaceInteractorModule:
    crate::private_functions::PrivateFunctionsModule
    +crate::storage::StorageModule
{
    /// Claim royalties from marketplace
    /// @param marketplace_address: the address of marketplace contract
    /// @param tokens: the list of token identifiers to claim
    /// @dev claim royalties from marketplace by calling claim_tokens function in nft marketplace contract
    ///      payable  ✔️non-payable
    ///      requires: - only can be called by admin
    #[endpoint(claimRoyaltiesFromMarketplace)]
    fn claim_royalties_from_marketplace(
        &self,
        marketplace_address: ManagedAddress,
        tokens: MultiValueEncoded<EgldOrEsdtTokenIdentifier>,
    ) {
        self.require_caller_is_admin();

        let mut args = MultiValueEncoded::new();
        for token in tokens {
            args.push((token, 0).into());
        }

        let own_sc_address = self.blockchain().get_sc_address();
        let call_result: EgldValuePaymentsVecPair<Self::Api> = self
            .nft_marketplace_proxy_builder(marketplace_address)
            .claim_tokens(own_sc_address, args)
            .execute_on_dest_context();

        let (egld_amount, other_payments) = call_result.into_tuple();
        if egld_amount > 0 {
            self.add_royalties(EgldOrEsdtTokenIdentifier::egld(), egld_amount);
        }
        if !other_payments.is_empty() {
            self.add_royalties_multiple(&other_payments)
        }
    }

    #[proxy]
    fn nft_marketplace_proxy_builder(
        &self,
        sc_address: ManagedAddress,
    ) -> nft_marketplace_proxy::Proxy<Self::Api>;
}
