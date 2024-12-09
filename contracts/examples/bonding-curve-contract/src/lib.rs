#![no_std]

testnumbat_wasm::imports!();
testnumbat_wasm::derive_imports!();

use testnumbat_wasm_module_bonding_curve::utils::{events, owner_endpoints, storage, user_endpoints};

#[testnumbat_wasm::contract]
pub trait Contract:
    testnumbat_wasm_module_bonding_curve::BondingCurveModule
    + storage::StorageModule
    + events::EventsModule
    + user_endpoints::UserEndpointsModule
    + owner_endpoints::OwnerEndpointsModule
{
    #[init]
    fn init(&self) {}
}
