#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;

testnumbat_wasm::imports!();

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - DcdtModule
/// - GovernanceModule
/// - PauseModule
#[testnumbat_wasm::contract]
pub trait UseModule:
    internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + testnumbat_wasm_module_dns::DnsModule
    + testnumbat_wasm_module_dcdt::DcdtModule
    + testnumbat_wasm_module_features::FeaturesModule
    + testnumbat_wasm_module_governance::GovernanceModule
    + testnumbat_wasm_module_governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + testnumbat_wasm_module_pause::PauseModule
{
    #[init]
    fn init(&self) {}

    /// Validates that the "featureName" feature is on.
    /// Uses the `feature_guard!` macro.
    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) {
        self.check_feature_on(b"featureName", true);
    }

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }
}
