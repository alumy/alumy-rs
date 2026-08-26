#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$ROOT_DIR"

run_check() {
    printf '\n==> %s\n' "$*"
    "$@"
}

run_check cargo fmt --all --check
run_check cargo test --workspace --all-features

run_check cargo test -p alumy --no-default-features --features bare
run_check cargo test -p alumy --no-default-features --features freertos
run_check cargo test -p alumy --no-default-features --features embassy

run_check cargo check -p alumy --features linux
run_check cargo check -p alumy --no-default-features --features bare
run_check cargo check -p alumy --no-default-features --features freertos
run_check cargo check -p alumy --no-default-features --features embassy

run_check cargo build -p alumy-example-linux
run_check cargo build -p alumy-example-bare --target thumbv7em-none-eabihf
run_check cargo build -p alumy-example-freertos --target thumbv7em-none-eabihf
run_check cargo build -p alumy-example-embassy --target thumbv7em-none-eabihf

run_check cargo doc --workspace --all-features --no-deps
run_check cargo publish --dry-run --allow-dirty -p alumy --registry crates-io

printf '\nRelease checks passed.\n'
