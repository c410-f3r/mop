#!/usr/bin/env bash

. "$(dirname "$0")/commons.sh" --source-only

test_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Testing ${package} without features *****\e[0m\n"
    cargo test --manifest-path "$(dirname "$0")/../${package}/Cargo.toml" --no-default-features

    /bin/echo -e "\e[0;33m***** Testing ${package} with all features *****\e[0m\n"
    cargo test --manifest-path "$(dirname "$0")/../${package}/Cargo.toml" --all-features
}

test_package_with_feature() {
    local package=$1
    local feature=$2

    /bin/echo -e "\e[0;33m***** Testing ${package} with feature '${feature}' *****\e[0m\n"
    cargo test --manifest-path "$(dirname "$0")/../${package}/Cargo.toml" --features "${feature}" --no-default-features
}

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
test_package_with_feature "mop-common" "std"
test_package_with_feature "mop-common" "with-futures"

test_package_generic "mop-facades"

test_package_generic "mop-solvers"

test_package_with_feature "mop-problems" "binh-and-korn"
test_package_with_feature "mop-problems" "constr"
test_package_with_feature "mop-problems" "cvrp"
test_package_with_feature "mop-problems" "rastrigin"
test_package_with_feature "mop-problems" "schaffer-function-2"
test_package_with_feature "mop-problems" "test-function-4"