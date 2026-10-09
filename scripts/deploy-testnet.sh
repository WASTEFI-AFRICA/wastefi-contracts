#!/usr/bin/env bash
# Build, deploy, initialize and wire all seven WasteFi contracts on Stellar testnet.
#
# Usage: scripts/deploy-testnet.sh
#
# Requires the Stellar CLI (`stellar`). The deployer is a CLI identity named by
# DEPLOYER (default: wastefi-deployer). It is created and funded through Friendbot
# if it does not exist. The secret key stays in the CLI's own key store; this script
# never prints it and never writes it into the repository.
#
# On success it writes deployed_addresses_testnet.json. Run scripts/smoke-test.sh
# afterwards to confirm the deployed contracts actually work.

set -euo pipefail
cd "$(dirname "$0")/.."

NETWORK=testnet
DEPLOYER="${DEPLOYER:-wastefi-deployer}"
WASM_DIR=target/wasm32v1-none/release
OUT=deployed_addresses_testnet.json

command -v stellar >/dev/null || { echo "The Stellar CLI (stellar) is required: https://developers.stellar.org/docs/tools/cli" >&2; exit 1; }

if ! stellar keys address "$DEPLOYER" >/dev/null 2>&1; then
  echo "Creating and funding testnet identity '$DEPLOYER'"
  stellar keys generate "$DEPLOYER" --network "$NETWORK" --fund
fi
ADMIN=$(stellar keys address "$DEPLOYER")
echo "Deployer / admin: $ADMIN"

./scripts/build-wasm.sh

# Run a CLI command, printing nothing on success and the full output on failure.
quietly() {
  local out
  if ! out=$("$@" 2>&1); then
    echo "$out" >&2
    return 1
  fi
  printf '%s\n' "$out"
}

deploy() {
  quietly stellar contract deploy --wasm "$WASM_DIR/$1.wasm" --source-account "$DEPLOYER" --network "$NETWORK" | tail -1
}

invoke() {
  local id=$1; shift
  quietly stellar contract invoke --id "$id" --source-account "$DEPLOYER" --network "$NETWORK" -- "$@" >/dev/null
}

echo
echo "Deploying"
TOKEN=$(deploy waste_token);              echo "  waste_token           $TOKEN"
REGISTRY=$(deploy collector_registry);    echo "  collector_registry    $REGISTRY"
POINTS=$(deploy collection_point);        echo "  collection_point      $POINTS"
PRICING=$(deploy material_pricing);       echo "  material_pricing      $PRICING"
REPUTATION=$(deploy reputation);          echo "  reputation            $REPUTATION"
TRANSACTION=$(deploy waste_transaction);  echo "  waste_transaction     $TRANSACTION"
PAYMENT=$(deploy payment_distribution);   echo "  payment_distribution  $PAYMENT"

echo
echo "Initializing"
invoke "$TOKEN" initialize --admin "$ADMIN" --name "WasteFi Token" --symbol WASTE --decimals 7
invoke "$REGISTRY" initialize --admin "$ADMIN"
invoke "$POINTS" initialize --admin "$ADMIN"
invoke "$PRICING" initialize --admin "$ADMIN"
invoke "$REPUTATION" initialize --admin "$ADMIN"
invoke "$TRANSACTION" initialize --admin "$ADMIN"
invoke "$PAYMENT" initialize --admin "$ADMIN" --token_contract "$TOKEN"

echo "Wiring waste_transaction to the contracts it calls"
invoke "$TRANSACTION" set_material_pricing_contract --contract_address "$PRICING"
invoke "$TRANSACTION" set_reputation_contract --contract_address "$REPUTATION"
invoke "$TRANSACTION" set_collector_registry_contract --contract_address "$REGISTRY"

# Record exactly what was deployed. A working tree with uncommitted changes is
# flagged, because the commit hash alone would not describe the deployed code.
SOURCE_COMMIT=$(git rev-parse --short HEAD)
git diff --quiet HEAD -- . ':!deployed_addresses_testnet.json' || SOURCE_COMMIT="$SOURCE_COMMIT+uncommitted"

cat > "$OUT" <<JSON
{
  "network": "$NETWORK",
  "deployment_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "source_commit": "$SOURCE_COMMIT",
  "deployer": "$ADMIN",
  "contracts": {
    "waste_token": "$TOKEN",
    "collector_registry": "$REGISTRY",
    "collection_point": "$POINTS",
    "material_pricing": "$PRICING",
    "reputation": "$REPUTATION",
    "waste_transaction": "$TRANSACTION",
    "payment_distribution": "$PAYMENT"
  }
}
JSON

echo
echo "Wrote $OUT. Next: scripts/smoke-test.sh"
