#!/usr/bin/env bash
# Build the contract wasm artifacts.
#
# Usage: scripts/build-wasm.sh [crate ...]    (default: every contract)
#
# Two details matter, and a plain `cargo build` gets both wrong:
#
# * The target is wasm32v1-none, not wasm32-unknown-unknown. Since Rust 1.82 the
#   latter emits WebAssembly features (reference types) that the Soroban VM
#   rejects, so the wasm builds but cannot be deployed: the network refuses it
#   with "reference-types not enabled". wasm32v1-none is the WebAssembly 1.0
#   target the Stellar tooling uses. It needs soroban-sdk 22 or later.
#
# * Each contract is built with an explicit `--crate-type cdylib`. Cargo.toml lists
#   `lib` as well, so the integration tests can import the contract crates, but a
#   default build then emits noticeably larger wasm, and wasm size is what
#   deployment fees are charged on. A bare `cargo build` at the workspace root
#   builds only the root package and emits no contract wasm at all.

set -euo pipefail

CONTRACTS=(
  collector_registry
  collection_point
  waste_token
  waste_transaction
  payment_distribution
  reputation
  material_pricing
)

cd "$(dirname "$0")/.."

targets=("$@")
if [ ${#targets[@]} -eq 0 ]; then
  targets=("${CONTRACTS[@]}")
fi

for crate in "${targets[@]}"; do
  echo "Building ${crate}"
  cargo rustc --quiet --package "$crate" --release \
    --target wasm32v1-none --crate-type cdylib
done

echo
echo "Built contracts:"
for crate in "${targets[@]}"; do
  wasm="target/wasm32v1-none/release/${crate}.wasm"
  bytes=$(wc -c < "$wasm")
  awk -v name="${crate}.wasm" -v bytes="$bytes" 'BEGIN { printf "  %-24s %7.1f KB\n", name, bytes / 1024 }'
done
