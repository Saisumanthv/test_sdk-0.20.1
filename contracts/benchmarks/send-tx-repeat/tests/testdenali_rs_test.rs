use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/send-tx-repeat.wasm",
        Box::new(|context| Box::new(send_tx_repeat::contract_obj(context))),
    );
    contract_map
}

#[test]
fn test_send_tx_repeat_without_data_testdenali_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/send_tx_repeat_without_data.scen.json",
        &contract_map(),
    );
}

#[test]
fn test_send_tx_repeat_with_data_testdenali_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/send_tx_repeat_with_data.scen.json", &contract_map());
}