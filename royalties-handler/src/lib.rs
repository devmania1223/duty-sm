#![no_std]

elrond_wasm::imports!();

pub mod common_storage;
pub mod nft_minter_interactor;
pub mod reward_entries;
pub mod shareholders;
pub mod token_balance;

#[elrond_wasm::contract]
pub trait RoyaltiesHandler:
    common_storage::CommonStorageModule
    + nft_minter_interactor::NftMinterInteractorModule
    + shareholders::ShareholdersModule
    + reward_entries::RewardEntriesModule
    + token_balance::TokenBalanceModule
{
    #[init]
    fn init(
        &self,
        nft_minter_sc_address: ManagedAddress,
    ) {
        require!(
            self.blockchain().is_smart_contract(&nft_minter_sc_address),
            "Invalid NFT Minter SC address"
        );

        self.nft_minter_sc_address().set(&nft_minter_sc_address);

        let mut sh_addresses = MultiValueEncoded::new();
        let percent: u32 = 100;
        sh_addresses.push((self.blockchain().get_caller(), BigUint::from(percent)).into());

        self.add_shareholders(sh_addresses);
    }
}
