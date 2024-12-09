testnumbat_wasm::imports!();

#[testnumbat_wasm::proxy]
pub trait Dns {
    #[payable("REWA")]
    #[endpoint]
    fn register(&self, name: BoxedBytes, #[payment] payment: BigUint);
}
