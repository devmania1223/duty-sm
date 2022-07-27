#![no_std]

elrond_wasm::imports!();

pub mod admin_whitelist;
pub mod brand_creation;
pub mod common_storage;
pub mod events;
pub mod nft_attributes_builder;
pub mod nft_marketplace_interactor;
pub mod nft_minting;
pub mod nft_tier;
pub mod royalties;
pub mod views;

#[elrond_wasm::contract]
pub trait NftMinter:
    common_storage::CommonStorageModule
    + admin_whitelist::AdminWhitelistModule
    + brand_creation::BrandCreationModule
    + nft_minting::NftMintingModule
    + nft_tier::NftTierModule
    + nft_attributes_builder::NftAttributesBuilderModule
    + royalties::RoyaltiesModule
    + nft_marketplace_interactor::NftMarketplaceInteractorModule
    + views::ViewsModule
    + events::EventsModule
{
    #[init]
    fn init(
        &self,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
    ) {
        self.royalties_claim_address().set(&royalties_claim_address);
        self.mint_payments_claim_address()
            .set(&mint_payments_claim_address);
    }
}
