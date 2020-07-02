#!/usr/bin/env bash

set -eux

PACKAGES=(
    mop-common
    mop-blocks
    mop-solvers
    mop-facades    
    mop
)

for package in "${PACKAGES[@]}"; do
    pushd "${package}"
    /bin/echo -e "\e[0;33m***** Publishing ${package} *****\e[0m\n"
    cargo publish
    sleep 20
    popd
done