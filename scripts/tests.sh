#!/usr/bin/env bash

set -euxo pipefail

cargo install --git https://github.com/c410-f3r/rust-tools --force

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags '' -Dmissing_docs)"

$rt rustfmt
$rt clippy -Aclippy::float_arithmetic,-Aclippy::indexing_slicing,-Aclippy::integer_arithmetic,-Aclippy::let_underscore_must_use,-Aclippy::panic,-Aclippy::type_complexity

$rt test-generic mop-bindings
$rt test-with-features mop-bindings with-futures
$rt test-with-features mop-bindings with-wasm_bindgen

$rt test-generic mop-blocks
$rt test-with-features mop-blocks std
$rt test-with-features mop-blocks with-futures
$rt test-with-features mop-blocks with-ndsparse
$rt test-with-features mop-blocks with-rand
$rt test-with-features mop-blocks with-serde

$rt test-generic mop-common
$rt test-with-features mop-common with-futures

$rt test-generic mop-facades

$rt test-generic mop-solvers

$rt check-with-features mop-problems binh-and-korn
$rt check-with-features mop-problems constr
$rt check-with-features mop-problems cvrp
$rt check-with-features mop-problems rastrigin
$rt check-with-features mop-problems schaffer-function-2
$rt check-with-features mop-problems test-function-4
