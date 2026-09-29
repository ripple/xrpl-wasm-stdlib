//! Runs the generator over a saved copy of rippled's real `lib.rs`
//! (`fixtures/rippled_host_functions_lib.rs`). This pins today's 63-function ABI as a
//! regression test and checks that every declaration in it can be lowered and emitted.

use std::collections::BTreeSet;

use generate_host_bindings::parse::{Return, parse_host_functions};
use generate_host_bindings::{Generated, generate};

const FIXTURE: &str = include_str!("fixtures/rippled_host_functions_lib.rs");

const EXPECTED_WASM_NAMES: [&str; 63] = [
    "accountroot_id",
    "amendment_enabled",
    "amm_id",
    "base_fee",
    "cache_le",
    "check_id",
    "check_sig",
    "credential_id",
    "delegate_id",
    "deposit_preauth_id",
    "did_id",
    "escrow_id",
    "float_add",
    "float_cmp",
    "float_div",
    "float_from_int",
    "float_from_mant_exp",
    "float_from_stamount",
    "float_from_stnumber",
    "float_from_uint",
    "float_mult",
    "float_pow",
    "float_sub",
    "float_to_int",
    "float_to_mant_exp",
    "home_le_arr_len",
    "home_le_field",
    "home_le_inner",
    "home_le_inner_arr_len",
    "ldgr_index",
    "le_arr_len",
    "le_field",
    "le_inner",
    "le_inner_arr_len",
    "loan_broker_id",
    "loan_id",
    "mpt_issuance_id",
    "mptoken_id",
    "nft_flags",
    "nft_issuer",
    "nft_offer_id",
    "nft_serial",
    "nft_taxon",
    "nft_uri",
    "nft_xfer_fee",
    "offer_id",
    "oracle_id",
    "parent_ldgr_hash",
    "parent_ldgr_time",
    "paychan_id",
    "permissioned_domain_id",
    "set_data",
    "sha512_half",
    "signers_id",
    "sponsorship_id",
    "ticket_id",
    "trace",
    "trustline_id",
    "tx_arr_len",
    "tx_field",
    "tx_inner",
    "tx_inner_arr_len",
    "vault_id",
];

#[test]
fn the_pinned_rippled_source_declares_exactly_these_63_host_functions() {
    let fns = parse_host_functions(FIXTURE).unwrap();
    let names: BTreeSet<&str> = fns.iter().map(|f| f.wasm_name.as_str()).collect();
    let expected: BTreeSet<&str> = EXPECTED_WASM_NAMES.into_iter().collect();
    assert_eq!(names, expected);
    assert_eq!(fns.first().unwrap().wasm_name, "ldgr_index");
    assert_eq!(fns.last().unwrap().wasm_name, "float_pow");
}

#[test]
fn trace_is_the_only_function_without_a_wasm_result() {
    let fns = parse_host_functions(FIXTURE).unwrap();
    let units: Vec<&str> = fns
        .iter()
        .filter(|f| f.returns == Return::Unit)
        .map(|f| f.wasm_name.as_str())
        .collect();
    assert_eq!(units, ["trace"]);
}

#[test]
fn every_declaration_generates_and_the_outputs_contain_every_name() {
    let Generated { trait_rs, list_rs } = generate(FIXTURE, "fixture").unwrap();
    for name in EXPECTED_WASM_NAMES {
        assert!(
            trait_rs.contains(&format!("unsafe fn {name}(&self")),
            "trait lacks {name}"
        );
        assert!(
            list_rs.contains(&format!("            fn {name}(")),
            "list lacks {name}"
        );
    }
    assert_eq!(trait_rs.matches("unsafe fn ").count(), 63);
    assert!(trait_rs.contains("// Source: fixture\n"));
    // rippled's doc links must have been rewritten or demoted.
    assert!(
        !trait_rs.contains("[`HostFunctions::"),
        "unrewritten HostFunctions link"
    );
    assert!(
        !trait_rs.contains("[`HASH_LEN`]"),
        "undemoted HASH_LEN link"
    );
}
