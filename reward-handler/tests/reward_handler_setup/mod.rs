use crate::nft_collection_setup::DutyNftMinterSetup;
use elrond_wasm::types::{Address, MultiValueEncoded, BigUint};
use elrond_wasm_debug::{
    managed_address, rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper},
    tx_mock::TxResult,
    DebugApi,
};
use nft_collection::DutyNftMinter;
use reward_handler::nft_collection_interactor::DutyNftMinterInteractorModule;
use reward_handler::DutyRewardHandler;

pub struct DutyRewardHandlerSetup<DutyRewardHandlerObjBuilder>
where
    DutyRewardHandlerObjBuilder: 'static + Copy + Fn() -> reward_handler::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_shareholder_address: Address,
    pub second_shareholder_address: Address,
    pub nft_collection_address: Address,
    pub rh_wrapper:
        ContractObjWrapper<reward_handler::ContractObj<DebugApi>, DutyRewardHandlerObjBuilder>,
}

impl<DutyRewardHandlerObjBuilder> DutyRewardHandlerSetup<DutyRewardHandlerObjBuilder>
where
    DutyRewardHandlerObjBuilder: 'static + Copy + Fn() -> reward_handler::ContractObj<DebugApi>,
{
    pub fn new<DutyNftMinterObjBuilder>(
        nm_builder: DutyNftMinterObjBuilder,
        rh_builder: DutyRewardHandlerObjBuilder,
    ) -> Self
    where
        DutyNftMinterObjBuilder: 'static + Copy + Fn() -> nft_collection::ContractObj<DebugApi>,
    {
        let rust_zero = rust_biguint!(0);
        let mut nm_setup = DutyNftMinterSetup::new(nm_builder);
        nm_setup.create_default_collections();

        let mut b_mock = nm_setup.b_mock;
        let owner_address = nm_setup.owner_address;
        let nm_wrapper = nm_setup.nm_wrapper;

        let first_shareholder_address = b_mock.create_user_account(&rust_zero);
        let second_shareholder_address = b_mock.create_user_account(&rust_zero);
        let percent: u32 = 5_000;

        // init royalties handler SC
        let rh_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner_address), rh_builder, "roy path");

        b_mock
            .execute_tx(&owner_address, &rh_wrapper, &rust_zero, |sc| {
                let mut sh_addresses = MultiValueEncoded::new();
                sh_addresses.push((managed_address!(&first_shareholder_address), BigUint::from(percent)).into());
                sh_addresses.push((managed_address!(&second_shareholder_address), BigUint::from(percent)).into());

                sc.init(managed_address!(nm_wrapper.address_ref()));

                sc.remove_shareholders();
                sc.add_shareholders(sh_addresses)
            })
            .assert_ok();

        // set the roylaties handler SC as the claim address
        b_mock
            .execute_tx(&owner_address, &nm_wrapper, &rust_zero, |sc| {
                sc.set_royalties_claim_address(managed_address!(rh_wrapper.address_ref()));
                sc.set_mint_payments_claim_address(managed_address!(rh_wrapper.address_ref()));
            })
            .assert_ok();

        Self {
            b_mock,
            owner_address,
            first_shareholder_address,
            second_shareholder_address,
            nft_collection_address: nm_wrapper.address_ref().clone(),
            rh_wrapper,
        }
    }

    pub fn call_claim_payments(&mut self) -> TxResult {
        self.b_mock.execute_tx(
            &self.owner_address,
            &self.rh_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_nft_collection_payments_and_royalties();
            },
        )
    }

    pub fn call_create_new_reward_entry(&mut self) -> TxResult {
        self.b_mock.execute_tx(
            &self.owner_address,
            &self.rh_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.create_new_reward_entry();
            },
        )
    }

    pub fn call_claim_rewards(&mut self, caller: &Address, entry_ids: &[usize]) -> TxResult {
        self.b_mock
            .execute_tx(&caller, &self.rh_wrapper, &rust_biguint!(0), |sc| {
                let mut args = MultiValueEncoded::new();
                for id in entry_ids {
                    args.push(*id);
                }

                sc.claim_rewards(args);
            })
    }
}
