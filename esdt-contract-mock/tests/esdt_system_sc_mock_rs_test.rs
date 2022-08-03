use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("esdt-contract-mock");
    blockchain.register_contract_builder(
        "file:output/esdt-contract-mock.wasm",
        esdt_contract_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn issue_rs() {
    elrond_wasm_debug::mandos_rs("mandos/esdt_contract.scen.json", world());
}
