use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/basic-features.wasm",
        Box::new(|context| Box::new(basic_features::contract_obj(context))),
    );
    contract_map
}

#[test]
fn big_int_to_i64_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/big_int_to_i64.scen.json", &contract_map());
}

#[test]
fn big_num_conversions_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/big_num_conversions.scen.json", &contract_map());
}

#[test]
fn big_uint_sqrt_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/big_uint_sqrt.scen.json", &contract_map());
}

#[test]
fn big_uint_to_u64_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/big_uint_to_u64.scen.json", &contract_map());
}

#[test]
fn block_info_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/block_info.scen.json", &contract_map());
}

#[test]
fn boxed_bytes_zeros_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/boxed_bytes_zeros.scen.json", &contract_map());
}

#[test]
fn codec_err_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/codec_err.scen.json", &contract_map());
}

#[test]
fn count_ones_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/count_ones.scen.json", &contract_map());
}

// #[test]
// fn crypto_elliptic_curves_rs() {
//     testnumbat_wasm_debug::testdenali_rs("testdenali/crypto_elliptic_curves.scen.json", &contract_map());
// }

#[test]
fn crypto_keccak256_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/crypto_keccak256.scen.json", &contract_map());
}

// #[test]
// fn crypto_ripemd160_rs() {
//     testnumbat_wasm_debug::testdenali_rs("testdenali/crypto_ripemd160.scen.json", &contract_map());
// }

#[test]
fn crypto_sha256_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/crypto_sha256.scen.json", &contract_map());
}

// #[test]
// fn crypto_verify_funcs_rs() {
//     testnumbat_wasm_debug::testdenali_rs("testdenali/crypto_verify_funcs.scen.json", &contract_map());
// }

#[test]
fn echo_array_u8_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_array_u8.scen.json", &contract_map());
}

#[test]
fn echo_async_result_empty_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_async_result_empty.scen.json", &contract_map());
}

#[test]
fn echo_async_result_empty_managed_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/echo_async_result_empty_managed.scen.json",
        &contract_map(),
    );
}

#[test]
fn echo_big_int_nested_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_big_int_nested.scen.json", &contract_map());
}

#[test]
fn echo_big_int_top_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_big_int_top.scen.json", &contract_map());
}

#[test]
fn echo_big_uint_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_big_uint.scen.json", &contract_map());
}

#[test]
fn echo_boxed_bytes_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_boxed_bytes.scen.json", &contract_map());
}

#[test]
fn echo_i32_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_i32.scen.json", &contract_map());
}

#[test]
fn echo_i64_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_i64.scen.json", &contract_map());
}

#[test]
fn echo_managed_bytes_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_managed_bytes.scen.json", &contract_map());
}

#[test]
fn echo_managed_vec_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_managed_vec.scen.json", &contract_map());
}

#[test]
fn echo_nothing_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_nothing.scen.json", &contract_map());
}

#[test]
fn echo_ser_ex_1_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_ser_ex_1.scen.json", &contract_map());
}

#[test]
fn echo_slice_u8_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_slice_u8.scen.json", &contract_map());
}

#[test]
fn echo_str_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_str.scen.json", &contract_map());
}

#[test]
fn echo_str_box_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_str_box.scen.json", &contract_map());
}

#[test]
fn echo_string_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_string.scen.json", &contract_map());
}

#[test]
fn echo_tuple_into_multiresult_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/echo_tuple_into_multiresult.scen.json",
        &contract_map(),
    );
}

#[test]
fn echo_u64_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_u64.scen.json", &contract_map());
}

#[test]
fn echo_usize_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_usize.scen.json", &contract_map());
}

#[test]
fn echo_varags_tuples_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_varags_tuples.scen.json", &contract_map());
}

#[test]
fn echo_varargs_u32_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_varargs_u32.scen.json", &contract_map());
}

#[test]
fn echo_vec_u8_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/echo_vec_u8.scen.json", &contract_map());
}

#[test]
fn events_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/events.scen.json", &contract_map());
}

#[test]
fn events_legacy_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/events_legacy.scen.json", &contract_map());
}

#[test]
fn get_caller_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/get_caller.scen.json", &contract_map());
}

#[test]
fn get_cumulated_validator_rewards_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/get_cumulated_validator_rewards.scen.json",
        &contract_map(),
    );
}

// TODO: uncomment after implemented the full DCDT format in testdenali-rs
// #[test]
// fn get_dcdt_local_roles_rs() {
// 	testnumbat_wasm_debug::testdenali_rs(
// 		"testdenali/get_dcdt_local_roles.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn managed_buffer_concat_1_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_buffer_concat_1.scen.json", &contract_map());
}

#[test]
fn managed_buffer_concat_2_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_buffer_concat_2.scen.json", &contract_map());
}

#[test]
fn managed_buffer_eq_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_buffer_eq.scen.json", &contract_map());
}

#[test]
fn managed_buffer_overwrite_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_buffer_overwrite.scen.json", &contract_map());
}

#[test]
fn managed_buffer_slice_1_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_buffer_slice_1.scen.json", &contract_map());
}

#[test]
fn managed_buffer_slice_2_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_buffer_slice_2.scen.json", &contract_map());
}

#[test]
fn managed_vec_address_push_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_vec_address_push.scen.json", &contract_map());
}

#[test]
fn managed_vec_biguint_push_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/managed_vec_biguint_push.scen.json", &contract_map());
}

#[test]
fn only_owner_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/only_owner.scen.json", &contract_map());
}

// Will never run in testdenali-rs.
// #[test]
// fn out_of_gas_rs() {
//     testnumbat_wasm_debug::testdenali_rs("testdenali/out_of_gas.scen.json", &contract_map());
// }

#[test]
fn panic_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/panic.scen.json", &contract_map());
}

#[test]
fn return_codes_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/return_codes.scen.json", &contract_map());
}

// TODO: Fix by implementing is_smart_contract mock
/*
#[test]
fn sc_properties_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/sc_properties.scen.json", &contract_map());
}
*/

#[test]
fn sc_result_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/sc_result.scen.json", &contract_map());
}

#[test]
fn storage_addr_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_addr.scen.json", &contract_map());
}

#[test]
fn storage_big_int_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_big_int.scen.json", &contract_map());
}

#[test]
fn storage_big_uint_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_big_uint.scen.json", &contract_map());
}

#[test]
fn storage_bool_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_bool.scen.json", &contract_map());
}

#[test]
fn storage_clear_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_clear.scen.json", &contract_map());
}

#[test]
fn storage_i64_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_i64.scen.json", &contract_map());
}

#[test]
fn storage_i64_bad_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_i64_bad.scen.json", &contract_map());
}

#[test]
fn storage_map1_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_map1.scen.json", &contract_map());
}

#[test]
fn storage_map2_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_map2.scen.json", &contract_map());
}

#[test]
fn storage_map3_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_map3.scen.json", &contract_map());
}

#[test]
fn storage_mapper_linked_list_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/storage_mapper_linked_list.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_queue_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_mapper_queue.scen.json", &contract_map());
}

#[test]
fn storage_mapper_map_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_mapper_map.scen.json", &contract_map());
}

#[test]
fn storage_mapper_map_storage_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/storage_mapper_map_storage.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_set_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_mapper_set.scen.json", &contract_map());
}

#[test]
fn storage_mapper_single_value_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/storage_mapper_single_value.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_token_attributes_rs() {
    testnumbat_wasm_debug::testdenali_rs(
        "testdenali/storage_mapper_token_attributes.scen.json",
        &contract_map(),
    );
}

#[test]
fn storage_mapper_vec_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_mapper_vec.scen.json", &contract_map());
}

#[test]
fn storage_opt_addr_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_opt_addr.scen.json", &contract_map());
}

#[test]
fn storage_reserved_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_reserved.scen.json", &contract_map());
}

#[test]
fn storage_u64_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_u64.scen.json", &contract_map());
}

#[test]
fn storage_u64_bad_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_u64_bad.scen.json", &contract_map());
}

#[test]
fn storage_usize_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_usize.scen.json", &contract_map());
}

#[test]
fn storage_usize_bad_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_usize_bad.scen.json", &contract_map());
}

#[test]
fn storage_vec_u8_rs() {
    testnumbat_wasm_debug::testdenali_rs("testdenali/storage_vec_u8.scen.json", &contract_map());
}
