#!/usr/bin/env bash

set -euxo pipefail

cargo install rust-tools --git https://github.com/c410-f3r/regular-crates

rt='rust-tools --template you-rust'

export CARGO_TARGET_DIR="$($rt target-dir)"
export RUST_BACKTRACE=1
export RUSTFLAGS="$($rt rust-flags '' -Dmissing_docs,-Dunstable_features,-Dunused_crate_dependencies,-Dvariant_size_differences)"

$rt rustfmt
#$rt clippy

$rt test-generic mop
$rt test-with-features mop ndstruct
$rt test-with-features mop rand
$rt test-with-features mop rayon
$rt test-with-features mop serde
$rt test-with-features mop std
$rt test-with-features mop wasm-bindgen
