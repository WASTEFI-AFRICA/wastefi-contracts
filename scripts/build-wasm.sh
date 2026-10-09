#!/usr/bin/env bash
# Build the contract wasm artifacts.
#
# Usage: scripts/build-wasm.sh [crate ...]    (default: every contract)
#
# Each contract is built with an explicit `--crate-type cdylib`. Cargo.toml lists
# `lib` as well, so the integration tests can import the contract crates, but a
# plain `cargo build` then emits wasm roughly 40% larger, and wasm size is what
# deployment fees are charged on. The Stellar CLI's `contract build` does the
# same. A bare `cargo build` at the workspace root builds only the root package
# and emits no contract wasm at all.

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
    --target wasm32-unknown-unknown --crate-type cdylib
done

echo
echo "Built contracts:"
for crate in "${targets[@]}"; do
  wasm="target/wasm32-unknown-unknown/release/${crate}.wasm"
  bytes=$(wc -c < "$wasm")
  awk -v name="${crate}.wasm" -v bytes="$bytes" 'BEGIN { printf "  %-24s %7.1f KB\n", name, bytes / 1024 }'
done
