#!/usr/bin/env bash

set -e

export RUSTFLAGS='-D warnings'
export RUST_BACKTRACE=full

run_package_with_feature() {
    local package=$1
    local feature=$2

    /bin/echo -e "\e[0;33m***** Running ${package} with feature '${feature}'  *****\e[0m\n"
    RUSTFLAGS="-C target-cpu=native" cargo run --features "${feature}" --manifest-path "${package}"/Cargo.toml --release
}

test_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} without features *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --no-default-features

    /bin/echo -e "\e[0;33m***** Testing ${package} with all features *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --all-features
}

test_package_with_feature() {
    local package=$1
    local feature=$2

    /bin/echo -e "\e[0;33m***** Testing ${package} with feature '${feature}' *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --features "${feature}" --no-default-features
}

test_package_generic "mop-bindings"
test_package_with_feature "mop-bindings" "with_futures"
test_package_with_feature "mop-bindings" "with_wasm_bindgen"

test_package_generic "mop-blocks"
test_package_with_feature "mop-blocks" "std"
test_package_with_feature "mop-blocks" "with_arrayvec"
test_package_with_feature "mop-blocks" "with_futures"
test_package_with_feature "mop-blocks" "with_ndsparse"
test_package_with_feature "mop-blocks" "with_rand"
test_package_with_feature "mop-blocks" "with_serde"

test_package_generic "mop-common-defs"
test_package_with_feature "mop-common-defs" "std"
test_package_with_feature "mop-common-defs" "with_futures"

test_package_generic "mop-facades"

test_package_generic "mop-solvers"

run_package_with_feature "problems" "binh_and_korn"
run_package_with_feature "problems" "constr"
run_package_with_feature "problems" "cvrp"
run_package_with_feature "problems" "rastrigin"
run_package_with_feature "problems" "schaffer_function_2"
run_package_with_feature "problems" "test_function_4"