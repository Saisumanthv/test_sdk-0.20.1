mod user_builtin {
    testnumbat_wasm::imports!();

    #[testnumbat_wasm::proxy]
    pub trait UserBuiltin {
        #[endpoint(SetUserName)]
        fn set_user_name(&self, name: &BoxedBytes) -> BigUint;
    }
}

mod dns_mock {
    testnumbat_wasm::imports!();

    #[testnumbat_wasm::contract]
    pub trait DnsMock {
        #[proxy]
        fn user_builtin_proxy(&self, to: ManagedAddress) -> super::user_builtin::Proxy<Self::Api>;

        #[payable("REWA")]
        #[endpoint]
        fn register(&self, name: BoxedBytes, #[payment] _payment: BigUint) -> AsyncCall {
            let address = self.blockchain().get_caller();
            self.user_builtin_proxy(address)
                .set_user_name(&name)
                .async_call()
        }
    }
}

use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/use-module.wasm",
        Box::new(|context| Box::new(use_module::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../test-wasm/dns.wasm",
        Box::new(|context| Box::new(dns_mock::contract_obj(context))),
    );

    contract_map
}

fn _gov_contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../../output/use-module.wasm",
        Box::new(|context| Box::new(use_module::contract_obj(context))),
    );

    contract_map
}

#[test]
fn use_module_dns_register_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/use_module_dns_register.scen.json", &contract_map());
}

#[test]
fn use_module_features_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/use_module_features.scen.json", &contract_map());
}

#[test]
fn use_module_internal_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/use_module_internal.scen.json", &contract_map());
}

#[test]
fn use_module_pause_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/use_module_pause.scen.json", &contract_map());
}

// Governance module tests

/*

#[test]
fn cancel_defeated_proposal_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/use_module_governance/cancel_defeated_proposal.scen.json",
        &gov_contract_map(),
    );
}

#[test]
fn change_configuration_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "/home/testnumbat/test_sdk-0.0.2/contracts/feature-tests/use-module/testdenali/use_module_governance/change_configuration.scen.json",
        &gov_contract_map(),
    );
}

#[test]
fn init_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/use_module_governance/init.scen.json",
        &gov_contract_map(),
    );
}

#[test]
fn invalid_proposals_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/use_module_governance/invalid_proposals.scen.json",
        &gov_contract_map(),
    );
}

#[test]
fn withdraw_governance_tokens_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/use_module_governance/withdraw_governance_tokens.scen.json",
        &gov_contract_map(),
    );
}

*/
