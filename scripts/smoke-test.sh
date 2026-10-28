#!/usr/bin/env bash
# Drive one delivery through the contracts listed in deployed_addresses_testnet.json:
# register a collector and a collection point, record a collection priced by
# material_pricing, verify it (which updates reputation), record the payment and
# complete it (which mints WASTE to the collector), and check every result.
#
# Usage: scripts/smoke-test.sh
#
# Exits non-zero on the first step that fails, so it can gate a release. It needs
# the deployer identity from scripts/deploy-testnet.sh. It creates and funds two
# throwaway testnet identities for the collector and the collection point, so the
# calls are signed by three different accounts as they would be in real use.

set -euo pipefail
cd "$(dirname "$0")/.."

NETWORK=testnet
DEPLOYER="${DEPLOYER:-wastefi-deployer}"
ADDRESSES=deployed_addresses_testnet.json

command -v stellar >/dev/null || { echo "The Stellar CLI (stellar) is required" >&2; exit 1; }
[ -f "$ADDRESSES" ] || { echo "$ADDRESSES not found; run scripts/deploy-testnet.sh first" >&2; exit 1; }

addr() { python3 -c "import json,sys; print(json.load(open('$ADDRESSES'))['contracts']['$1'])"; }
TOKEN=$(addr waste_token);           REGISTRY=$(addr collector_registry)
POINTS=$(addr collection_point);     TRANSACTION=$(addr waste_transaction)
PAYMENT=$(addr payment_distribution); REPUTATION=$(addr reputation)

# Fresh throwaway identities every run, so the script is repeatable and never
# depends on state left by an earlier run. They are removed on exit.
RUN_ID="$RANDOM$RANDOM"
COLLECTOR_KEY="wastefi-smoke-collector-$RUN_ID"
POINT_KEY="wastefi-smoke-point-$RUN_ID"
VICTIM_KEY="wastefi-smoke-victim-$RUN_ID"
cleanup() {
  for k in "$COLLECTOR_KEY" "$POINT_KEY" "$VICTIM_KEY"; do
    stellar keys rm "$k" --force >/dev/null 2>&1 || true
  done
}
trap cleanup EXIT
for who in "$COLLECTOR_KEY" "$POINT_KEY"; do
  stellar keys generate "$who" --network "$NETWORK" --fund >/dev/null 2>&1
done
COLLECTOR=$(stellar keys address "$COLLECTOR_KEY")
POINT=$(stellar keys address "$POINT_KEY")

step=0
# call <source identity> <contract id> <function> [args...]; prints the return value.
call() {
  local source=$1 id=$2; shift 2
  local out
  if ! out=$(stellar contract invoke --id "$id" --source-account "$source" --network "$NETWORK" -- "$@" 2>&1); then
    echo "FAILED: $*" >&2
    echo "$out" | tail -5 >&2
    exit 1
  fi
  printf '%s\n' "$out" | tail -1 | tr -d '"'
}
ok() { step=$((step + 1)); printf '%2d. %s\n' "$step" "$1"; }

# Weight varies per run: identical repeat submissions are rejected as duplicates.
WEIGHT=$((1000 + RANDOM % 9000))
PRICE=$(call "$DEPLOYER" "$(addr material_pricing)" get_price --material_type '"Plastic"')
AMOUNT=$((WEIGHT * PRICE / 1000))

echo "Collector $COLLECTOR"
echo "Point     $POINT"
echo "Delivery  ${WEIGHT} g of Plastic at ${PRICE} per kg = ${AMOUNT}"
echo

call "$COLLECTOR_KEY" "$REGISTRY" register --collector "$COLLECTOR" --name "Smoke Collector" --phone "+254700000001" >/dev/null
ok "collector registered"
call "$DEPLOYER" "$REGISTRY" update_status --collector "$COLLECTOR" --status '"Active"' >/dev/null
[ "$(call "$DEPLOYER" "$REGISTRY" is_active --collector "$COLLECTOR")" = "true" ] || { echo "collector not active" >&2; exit 1; }
ok "admin activated the collector"

POINT_ID=$(call "$POINT_KEY" "$POINTS" register_point --owner "$POINT" --name "Smoke Point" --location "Testnet" --accepted_materials '["Plastic","Glass"]')
ok "collection point registered (id $POINT_ID)"
call "$DEPLOYER" "$POINTS" verify_point --point_id "$POINT_ID" >/dev/null
ok "admin verified the collection point"

TX_ID=$(call "$COLLECTOR_KEY" "$TRANSACTION" record_with_price_lookup --collector "$COLLECTOR" --collection_point "$POINT" --material_type '"Plastic"' --weight "$WEIGHT")
ok "collection recorded by the collector at the on-chain price (transaction $TX_ID)"

# Authorization. The Stellar CLI signs any authorization it holds a key for, so the
# victim must be an account whose key the CLI no longer has. Register and activate
# a throwaway collector, delete its key, then try to record a collection in its name
# from another account. Without the collector's signature the contract must refuse.
stellar keys generate "$VICTIM_KEY" --network "$NETWORK" --fund >/dev/null 2>&1
VICTIM=$(stellar keys address "$VICTIM_KEY")
call "$VICTIM_KEY" "$REGISTRY" register --collector "$VICTIM" --name "Smoke Victim" --phone "+254700000002" >/dev/null
call "$DEPLOYER" "$REGISTRY" update_status --collector "$VICTIM" --status '"Active"' >/dev/null
stellar keys rm "$VICTIM_KEY" --force >/dev/null 2>&1
if stellar contract invoke --id "$TRANSACTION" --source-account "$POINT_KEY" --network "$NETWORK" -- record_with_price_lookup --collector "$VICTIM" --collection_point "$POINT" --material_type '"Plastic"' --weight "$((WEIGHT + 777))" >/dev/null 2>&1; then
  echo "FAILED: a collection was recorded in an active collector's name without their signature" >&2
  exit 1
fi
ok "recording in an active collector's name without their signature was rejected"

call "$DEPLOYER" "$TRANSACTION" verify_transaction --transaction_id "$TX_ID" >/dev/null
ok "admin verified the transaction"

STATS=$(call "$DEPLOYER" "$REPUTATION" get_statistics --collector "$COLLECTOR")
echo "$STATS" | grep -qE '^\[[0-9]+,1,1,0\]$' || { echo "reputation did not record the verified transaction: $STATS" >&2; exit 1; }
ok "verification updated the collector's reputation ($STATS)"

if stellar contract invoke --id "$TRANSACTION" --source-account "$DEPLOYER" --network "$NETWORK" -- verify_transaction --transaction_id "$TX_ID" >/dev/null 2>&1; then
  echo "FAILED: the same transaction was verified twice" >&2
  exit 1
fi
ok "verifying the same transaction again was rejected"

STORED=$(call "$DEPLOYER" "$TRANSACTION" get_transaction --transaction_id "$TX_ID")
echo "$STORED" | grep -qE "total_amount:$AMOUNT[,}]" || { echo "stored amount is not $AMOUNT: $STORED" >&2; exit 1; }
ok "stored payout is exactly ${AMOUNT} (fractional kilograms are paid)"

PAYMENT_ID=$(call "$DEPLOYER" "$PAYMENT" process_payment --transaction_id "$TX_ID" --recipient "$COLLECTOR" --amount "$AMOUNT")
ok "payment recorded (id $PAYMENT_ID)"

if stellar contract invoke --id "$PAYMENT" --source-account "$DEPLOYER" --network "$NETWORK" -- process_payment --transaction_id "$TX_ID" --recipient "$COLLECTOR" --amount "$AMOUNT" >/dev/null 2>&1; then
  echo "FAILED: a second payment was recorded for the same transaction" >&2
  exit 1
fi
ok "a second payment for the same transaction was rejected"

BEFORE=$(call "$DEPLOYER" "$TOKEN" balance --account "$COLLECTOR")
call "$DEPLOYER" "$PAYMENT" update_payment_status --payment_id "$PAYMENT_ID" --status '"Completed"' >/dev/null
AFTER=$(call "$DEPLOYER" "$TOKEN" balance --account "$COLLECTOR")
[ "$((AFTER - BEFORE))" -eq "$AMOUNT" ] || { echo "balance moved by $((AFTER - BEFORE)), expected $AMOUNT" >&2; exit 1; }
ok "completing the payment minted ${AMOUNT} WASTE base units (7 decimals) to the collector"

echo
echo "Smoke test passed: $step checks against the deployed contracts."
