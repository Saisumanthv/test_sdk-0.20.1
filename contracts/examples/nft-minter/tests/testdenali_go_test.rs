#[test]
fn init_go() {
    testnumbat_wasm_debug::testdenali_go("testdenali/init.scen.json");
}

#[test]
fn create_nft_go() {
    testnumbat_wasm_debug::testdenali_go("testdenali/create_nft.scen.json");
}

#[test]
fn buy_nft_go() {
    testnumbat_wasm_debug::testdenali_go("testdenali/buy_nft.scen.json");
}
