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