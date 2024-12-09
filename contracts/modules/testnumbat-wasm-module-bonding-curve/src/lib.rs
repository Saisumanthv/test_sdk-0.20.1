#![no_std]

testnumbat_wasm::imports!();
testnumbat_wasm::derive_imports!();

pub mod curves;
pub mod function_selector;
pub mod utils;
use crate::utils::{events, owner_endpoints, storage, user_endpoints};

#[testnumbat_wasm::module]
pub trait BondingCurveModule:
    storage::StorageModule
    + events::EventsModule
    + user_endpoints::UserEndpointsModule
    + owner_endpoints::OwnerEndpointsModule
{
}
