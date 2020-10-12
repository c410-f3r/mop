#!/usr/bin/env bash

set -eux

export RUST_BACKTRACE=full
export RUSTFLAGS='
    -D bad_style
    -D future_incompatible
    -D missing_debug_implementations
    -D nonstandard_style
    -D rust_2018_compatibility
    -D rust_2018_idioms
    -D trivial_casts
    -D unsafe_code
    -D unused_lifetimes
    -D unused_qualifications
    -D warnings
'

clippy() {
    local package=$1
    local features=$2

    /bin/echo -e "\e[0;33m***** Running clippy on ${package} | ${features} *****\e[0m\n"
    cargo clippy $features --lib --manifest-path "${package}"/Cargo.toml -- \
        -D clippy::restriction \
        -D warnings \
        -A clippy::float_arithmetic \
        -A clippy::implicit_return \
        -A clippy::indexing_slicing \
        -A clippy::integer_arithmetic \
        -A clippy::let_underscore_must_use \
        -A clippy::missing_docs_in_private_items \
        -A clippy::missing_inline_in_public_items \
        -A clippy::panic \
        -A clippy::type_complexity
}

test_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} | --no-default-features *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --no-default-features

    clippy $package "--no-default-features"

    /bin/echo -e "\e[0;33m***** Testing ${package} | --all-features *****\e[0m\n"
    cargo test --all-features --manifest-path "${package}"/Cargo.toml

    clippy $package "--all-features"
}

test_package_with_feature() {
    local package=$1
    local features=$2

    /bin/echo -e "\e[0;33m***** Testing ${package} with feature '${features}' *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --features "${features}" --no-default-features

    clippy $package "--features ${features}"
}

cargo fmt --all -- --check

test_package_generic "mop-bindings"
test_package_with_feature "mop-bindings" "with-futures"
test_package_with_feature "mop-bindings" "with-wasm_bindgen"

test_package_generic "mop-blocks"
test_package_with_feature "mop-blocks" "std"
test_package_with_feature "mop-blocks" "with-futures"
test_package_with_feature "mop-blocks" "with-ndsparse"
test_package_with_feature "mop-blocks" "with-rand"
test_package_with_feature "mop-blocks" "with-serde"

test_package_generic "mop-common"
test_package_with_feature "mop-common" "with-futures"

test_package_generic "mop-facades"

test_package_generic "mop-solvers"

test_package_with_feature "mop-problems" "binh-and-korn"
test_package_with_feature "mop-problems" "constr"
test_package_with_feature "mop-problems" "cvrp"
test_package_with_feature "mop-problems" "rastrigin"
test_package_with_feature "mop-problems" "schaffer-function-2"
test_package_with_feature "mop-problems" "test-function-4"