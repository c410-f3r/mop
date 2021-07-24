#!/usr/bin/env bash

set -euxo pipefail

cargo fuzz run --fuzz-dir mop-blocks-fuzz dr_matrix -- -max_len=32 -runs=10000
cargo fuzz run --fuzz-dir mop-blocks-fuzz gp -- -max_len=32 -runs=10000