#!/usr/bin/env bash

set -euxo pipefail

cargo fuzz run --features libfuzzer-sys/link_libfuzzer --fuzz-dir mop-fuzz dr_matrix -- -max_len=32 -runs=10000
cargo fuzz run --features libfuzzer-sys/link_libfuzzer --fuzz-dir mop-fuzz gp -- -max_len=32 -runs=10000