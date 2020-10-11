#!/usr/bin/env bash

. "$(dirname "$0")/commons.sh" --source-only

cargo fmt --all -- --check

cargo clippy --all-features --lib -- \
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