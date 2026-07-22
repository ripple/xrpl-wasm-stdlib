#!/bin/bash
# Splits the ricky122-5/contract branch into 8 small, stacked, independently
# reviewable PR branches. Run this yourself (not via an agent) from the root
# of a real checkout of this repo, so your GPG signing key can sign each
# commit interactively.
#
# Assumptions:
#   - You're on a clean working tree (no uncommitted changes) when you start.
#   - `origin/main` and `ricky122-5/contract` are both resolvable refs in
#     this repo (fetch first if needed: `git fetch origin`).
#   - Run from the repo root.
#
# Each branch stacks on the previous one. Verified (cargo check/test, plus
# clippy.sh) cumulatively before this script was written -- branches 1-3 only
# touch xrpl-wasm-stdlib; branches 4-8 build up xrpl-contract-stdlib +
# xrpl-parameter-macro + e2e-tests/smart-contracts/*.

set -euo pipefail

SOURCE_BRANCH="ricky122-5/contract"
BASE="origin/main"

echo "=== Branch 1/8: split/01-sflags ==="
git checkout -b split/01-sflags "$BASE"
git checkout "$SOURCE_BRANCH" -- \
  xrpl-wasm-stdlib/src/sflags.rs \
  scripts/generate-sflags.sh \
  tools/generateSFlags.js \
  xrpl-wasm-stdlib/src/lib.rs
git add -A
git commit -m "Add sflags.rs (XRPL transaction/ledger-object flag constants) and its rippled-source generator script"

echo "=== Branch 2/8: split/02-common-types ==="
git checkout -b split/02-common-types split/01-sflags
git checkout "$SOURCE_BRANCH" -- \
  xrpl-wasm-stdlib/src/core/type_codes.rs \
  xrpl-wasm-stdlib/src/core/mod.rs \
  xrpl-wasm-stdlib/src/core/types/number.rs \
  xrpl-wasm-stdlib/src/core/types/mod.rs \
  xrpl-wasm-stdlib/src/core/types/amount.rs \
  xrpl-wasm-stdlib/src/sfield.rs \
  tools/generateSFields.js \
  scripts/generate-sfields.sh
git add -A
git commit -m "Add STI_* type codes, Number type, Contract SField constants, and Amount changes to xrpl-wasm-stdlib"

echo "=== Branch 3/8: split/03-host-bindings ==="
git checkout -b split/03-host-bindings split/02-common-types
git checkout "$SOURCE_BRANCH" -- \
  xrpl-wasm-stdlib/src/host/host_bindings_trait.rs \
  xrpl-wasm-stdlib/src/host/host_bindings_wasm.rs \
  xrpl-wasm-stdlib/src/host/host_bindings_empty.rs \
  xrpl-wasm-stdlib/src/host/host_bindings_test.rs \
  tools/compareHostFunctions.js
git add -A
git commit -m "Add contract-only host function bindings (data storage, txn build/emit, params) to HostBindings trait"

echo "=== Branch 4/8: split/04-contract-genesis ==="
git checkout -b split/04-contract-genesis split/03-host-bindings
git checkout "$SOURCE_BRANCH" -- \
  xrpl-contract-stdlib/Cargo.toml \
  xrpl-contract-stdlib/README.md \
  xrpl-contract-stdlib/src/sfield.rs \
  xrpl-contract-stdlib/src/sflags.rs \
  xrpl-contract-stdlib/src/current_tx/mod.rs \
  xrpl-contract-stdlib/src/current_tx/contract_call.rs \
  xrpl-contract-stdlib/src/current_tx/traits.rs \
  xrpl-contract-stdlib/src/submit/mod.rs \
  xrpl-contract-stdlib/src/submit/emit.rs \
  xrpl-contract-stdlib/src/submit/amount.rs \
  xrpl-contract-stdlib/src/submit/inner_objects.rs \
  xrpl-contract-stdlib/xrpl-parameter-macro/Cargo.toml \
  xrpl-contract-stdlib/xrpl-parameter-macro/src/lib.rs \
  scripts/check-wasm-exports.sh \
  e2e-tests/README.md \
  e2e-tests/smart-contracts/emit_txn

cat > Cargo.toml <<'EOF'
[workspace]
resolver = "2"
members = [
    "xrpl-wasm-stdlib",
    "xrpl-macros",
    "xrpl-escrow-stdlib",
    "xrpl-contract-stdlib",
    "xrpl-contract-stdlib/xrpl-parameter-macro",
]
exclude = [
    "examples",
    "e2e-tests",
]

[workspace.package]
license = "ISC"
repository = "https://github.com/ripple/xrpl-wasm-stdlib"

[profile.release]
opt-level = "s"   # Optimize for size
lto = true        # Link-time optimization
codegen-units = 1 # Compile in one unit for better optimization and smaller binary size
panic = "abort"   # `no-std` can't unwind on panic, so choose abort (plus no panic handler means smaller binary)

[profile.dev]
panic = "unwind" # For debugging, allows unwinding if code is executed as Rust (e.g., unit tests)
EOF

mkdir -p xrpl-contract-stdlib/src
cat > xrpl-contract-stdlib/src/lib.rs <<'EOF'
#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![allow(non_upper_case_globals)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod current_tx;
pub mod sfield;
pub mod sflags;
pub mod submit;

pub use xrpl_parameter_macro::wasm_export;
pub use xrpl_wasm_stdlib::*;
EOF

cat > e2e-tests/Cargo.toml <<'EOF'
[workspace]
resolver = "2"
members = [
    "float_tests",
    "gas_benchmark",
    "host_functions_test",
    "keylet_exists",
    "test_utils",
    "trace_escrow_account",
    "trace_escrow_finish",
    "trace_escrow_ledger_object",
    "smart-contracts/emit_txn",
]

[workspace.dependencies]
xrpl-wasm-stdlib = { path = "../xrpl-wasm-stdlib" }
xrpl-contract-stdlib = { path = "../xrpl-contract-stdlib" }
test_utils = { path = "test_utils" }

[profile.release]
opt-level = "s"   # Optimize for size
lto = true        # Link-time optimization
codegen-units = 1 # Compile in one unit for better optimization and smaller binary size
panic = "abort"   # `no-std` can't unwind on panic, so choose abort (plus no panic handler means smaller binary)

[profile.dev]
panic = "unwind" # For debugging, allows unwinding if code is executed as Rust (e.g., unit tests)
EOF

cargo check --workspace >/dev/null
cargo check --workspace --target wasm32-unknown-unknown --manifest-path e2e-tests/Cargo.toml >/dev/null
git add -A
git commit -m "Add xrpl-contract-stdlib crate (transaction submission + inner-object encoding) and xrpl-parameter-macro"

echo "=== Branch 5/8: split/05-params ==="
git checkout -b split/05-params split/04-contract-genesis
git checkout "$SOURCE_BRANCH" -- \
  xrpl-contract-stdlib/src/params/mod.rs \
  xrpl-contract-stdlib/src/params/function.rs \
  xrpl-contract-stdlib/src/params/instance.rs \
  xrpl-contract-stdlib/src/params/types.rs \
  e2e-tests/smart-contracts/amount_tests \
  e2e-tests/smart-contracts/function_params \
  e2e-tests/smart-contracts/instance_params_uint \
  e2e-tests/smart-contracts/instance_params_other \
  e2e-tests/smart-contracts/parameter_macro

cat > xrpl-contract-stdlib/src/lib.rs <<'EOF'
#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![allow(non_upper_case_globals)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod current_tx;
pub mod params;
pub mod sfield;
pub mod sflags;
pub mod submit;

pub use xrpl_parameter_macro::wasm_export;
pub use xrpl_wasm_stdlib::*;
EOF

cat > e2e-tests/Cargo.toml <<'EOF'
[workspace]
resolver = "2"
members = [
    "float_tests",
    "gas_benchmark",
    "host_functions_test",
    "keylet_exists",
    "test_utils",
    "trace_escrow_account",
    "trace_escrow_finish",
    "trace_escrow_ledger_object",
    "smart-contracts/amount_tests",
    "smart-contracts/emit_txn",
    "smart-contracts/function_params",
    "smart-contracts/instance_params_uint",
    "smart-contracts/instance_params_other",
    "smart-contracts/parameter_macro",
]

[workspace.dependencies]
xrpl-wasm-stdlib = { path = "../xrpl-wasm-stdlib" }
xrpl-contract-stdlib = { path = "../xrpl-contract-stdlib" }
test_utils = { path = "test_utils" }

[profile.release]
opt-level = "s"   # Optimize for size
lto = true        # Link-time optimization
codegen-units = 1 # Compile in one unit for better optimization and smaller binary size
panic = "abort"   # `no-std` can't unwind on panic, so choose abort (plus no panic handler means smaller binary)

[profile.dev]
panic = "unwind" # For debugging, allows unwinding if code is executed as Rust (e.g., unit tests)
EOF

cargo check --workspace >/dev/null
cargo check --workspace --target wasm32-unknown-unknown --manifest-path e2e-tests/Cargo.toml >/dev/null
git add -A
git commit -m "Add function/instance parameter extraction (params module) to xrpl-contract-stdlib"

echo "=== Branch 6/8: split/06-events ==="
git checkout -b split/06-events split/05-params
git checkout "$SOURCE_BRANCH" -- \
  xrpl-contract-stdlib/src/event/mod.rs \
  xrpl-contract-stdlib/src/event/codec_v2.rs \
  xrpl-contract-stdlib/src/event/codec_v3.rs \
  e2e-tests/smart-contracts/events

cat > xrpl-contract-stdlib/src/lib.rs <<'EOF'
#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![allow(non_upper_case_globals)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod current_tx;
pub mod event;
pub mod params;
pub mod sfield;
pub mod sflags;
pub mod submit;

pub use xrpl_parameter_macro::wasm_export;
pub use xrpl_wasm_stdlib::*;
EOF

cat > e2e-tests/Cargo.toml <<'EOF'
[workspace]
resolver = "2"
members = [
    "float_tests",
    "gas_benchmark",
    "host_functions_test",
    "keylet_exists",
    "test_utils",
    "trace_escrow_account",
    "trace_escrow_finish",
    "trace_escrow_ledger_object",
    "smart-contracts/amount_tests",
    "smart-contracts/emit_txn",
    "smart-contracts/events",
    "smart-contracts/function_params",
    "smart-contracts/instance_params_uint",
    "smart-contracts/instance_params_other",
    "smart-contracts/parameter_macro",
]

[workspace.dependencies]
xrpl-wasm-stdlib = { path = "../xrpl-wasm-stdlib" }
xrpl-contract-stdlib = { path = "../xrpl-contract-stdlib" }
test_utils = { path = "test_utils" }

[profile.release]
opt-level = "s"   # Optimize for size
lto = true        # Link-time optimization
codegen-units = 1 # Compile in one unit for better optimization and smaller binary size
panic = "abort"   # `no-std` can't unwind on panic, so choose abort (plus no panic handler means smaller binary)

[profile.dev]
panic = "unwind" # For debugging, allows unwinding if code is executed as Rust (e.g., unit tests)
EOF

cargo check --workspace >/dev/null
cargo check --workspace --target wasm32-unknown-unknown --manifest-path e2e-tests/Cargo.toml >/dev/null
git add -A
git commit -m "Add event emission codecs (v2/v3) to xrpl-contract-stdlib"

echo "=== Branch 7/8: split/07-data ==="
git checkout -b split/07-data split/06-events
git checkout "$SOURCE_BRANCH" -- \
  xrpl-contract-stdlib/src/data/mod.rs \
  xrpl-contract-stdlib/src/data/codec.rs \
  e2e-tests/smart-contracts/contract_data

cat > xrpl-contract-stdlib/src/lib.rs <<'EOF'
#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![allow(non_upper_case_globals)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod current_tx;
pub mod data;
pub mod event;
pub mod params;
pub mod sfield;
pub mod sflags;
pub mod submit;

pub use xrpl_parameter_macro::wasm_export;
pub use xrpl_wasm_stdlib::*;
EOF

cat > e2e-tests/Cargo.toml <<'EOF'
[workspace]
resolver = "2"
members = [
    "float_tests",
    "gas_benchmark",
    "host_functions_test",
    "keylet_exists",
    "test_utils",
    "trace_escrow_account",
    "trace_escrow_finish",
    "trace_escrow_ledger_object",
    "smart-contracts/amount_tests",
    "smart-contracts/contract_data",
    "smart-contracts/emit_txn",
    "smart-contracts/events",
    "smart-contracts/function_params",
    "smart-contracts/instance_params_uint",
    "smart-contracts/instance_params_other",
    "smart-contracts/parameter_macro",
]

[workspace.dependencies]
xrpl-wasm-stdlib = { path = "../xrpl-wasm-stdlib" }
xrpl-contract-stdlib = { path = "../xrpl-contract-stdlib" }
test_utils = { path = "test_utils" }

[profile.release]
opt-level = "s"   # Optimize for size
lto = true        # Link-time optimization
codegen-units = 1 # Compile in one unit for better optimization and smaller binary size
panic = "abort"   # `no-std` can't unwind on panic, so choose abort (plus no panic handler means smaller binary)

[profile.dev]
panic = "unwind" # For debugging, allows unwinding if code is executed as Rust (e.g., unit tests)
EOF

cargo check --workspace >/dev/null
cargo check --workspace --target wasm32-unknown-unknown --manifest-path e2e-tests/Cargo.toml >/dev/null
git add -A
git commit -m "Add contract data storage codec (get/set ContractData by account+key) to xrpl-contract-stdlib"

echo "=== Branch 8/8: split/08-ctx ==="
git checkout -b split/08-ctx split/07-data
git checkout "$SOURCE_BRANCH" -- \
  xrpl-contract-stdlib/src/ctx/mod.rs \
  xrpl-contract-stdlib/src/ctx/contract_call.rs

cat > xrpl-contract-stdlib/src/lib.rs <<'EOF'
#![doc = include_str!("../README.md")]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![allow(non_upper_case_globals)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

pub mod ctx;
pub mod current_tx;
pub mod data;
pub mod event;
pub mod params;
pub mod sfield;
pub mod sflags;
pub mod submit;

pub use ctx::{ContractCallContext, ContractStorage};
pub use xrpl_parameter_macro::wasm_export;
pub use xrpl_wasm_stdlib::*;
EOF

cargo check --workspace >/dev/null
cargo test -p xrpl-contract-stdlib >/dev/null
git add -A
git commit -m "Add ContractCallContext (storage/user_storage/emit) integration layer to xrpl-contract-stdlib"

echo "=== Done. Branches created: ==="
git log --oneline split/01-sflags..split/08-ctx
echo
echo "Review each with e.g.:"
echo "  git diff split/03-host-bindings..split/04-contract-genesis --stat"
echo
echo "Push and open PRs (base branch shown) once you're happy, e.g.:"
echo "  git push -u origin split/01-sflags"
echo "  gh pr create --base main --head split/01-sflags --title '...'"
echo "  git push -u origin split/02-common-types"
echo "  gh pr create --base split/01-sflags --head split/02-common-types --title '...'"
echo "  ...and so on, re-basing each PR's base onto main as the ones before it merge."
