#[test]
fn send_tx_repeat_without_data_go() {
    testnumbat_wasm_debug::testdenali_go("testdenali/send_tx_repeat_without_data.scen.json");
}

#[test]
fn send_tx_repeat_with_data_go() {
    testnumbat_wasm_debug::testdenali_go("testdenali/send_tx_repeat_with_data.scen.json");
}
