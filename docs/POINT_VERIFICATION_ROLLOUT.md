# Point verification on testnet

This rollout keeps the existing WASTE token and balances. It deploys a new
collection point registry, transaction contract and payment distribution contract.
The collector registry, pricing and reputation contracts stay in place. A fresh
transaction contract starts its IDs again, so it must use a fresh payment map.

Preparing the contracts and switching the hosted application are separate steps.
The preparation script never rewrites `deployed_addresses_testnet.json` or changes
Render, Vercel or the database. Its output is a candidate bundle, not evidence
that the hosted app uses point verification.

## Prepare the candidate

Requires Python 3 on Linux or macOS, Stellar CLI 27, the existing
`wastefi-deployer` identity and the current testnet manifest. No secret key is
passed to the script. CLI calls use the fixed public testnet RPC and network
passphrase. Do not run the seven-contract deploy script for this rollout: that
would create a different token with different balances.

```sh
scripts/build-wasm.sh collection_point waste_transaction payment_distribution
python3 scripts/rollout-point-verification-testnet.py
python3 scripts/rollout-point-verification-testnet.py --apply
```

The first Python command only prints the plan. The second checks the original
admins, pause flags, payment token and legacy roles before writing. It saves
random salts and deterministic contract IDs before deploying anything. It checks
the deployed WASM hashes, initializes the contracts, wires dependencies and adds
the new reputation operator and token minter. It retains the old operator and
minter so saved legacy collections can finish.

The default output is the ignored `target/point-verification-testnet.json`.
`--base`, `--out` and `--wasm-dir` can select other local paths. The output cannot
overwrite the active manifest. Back it up after preparation. Retain the journal
and rerun the same command after an interruption. Never delete it to retry:
deterministic IDs let a retry discover deployments whose response was lost.
Changed WASM, a changed base bundle or mismatched deployed code requires review.
Another process cannot use the same journal concurrently.

A completed journal says `ready_for_app_cutover`. This means its wiring and roles
were checked, not that the API has switched or that an end-to-end delivery passed.
Before cutover, exercise an actual point-signed collection and a saved legacy
collection using a disposable local database. Confirm both mint the existing
WASTE token and that repeating completed work does not mint again.

## Pin historical collections before switching the API

The backend's `stellar:pin-collections` command saves original public contract
addresses and signing addresses on old collections that already made on-chain
progress. This is needed even if there are no contract-related Render overrides.
Without it, the new API deliberately holds those collections for review.

Use the current backend instructions under
[Stellar: recording collections on-chain](https://github.com/WASTEFI-AFRICA/wastefi-backend#stellar-recording-collections-on-chain).
Stop all API workers, apply migrations, preview the backfill against the original
bundle, then apply with the original transaction and payment addresses explicitly
confirmed. Preview again until no historical rows remain unpinned. Investigate
every `needs_review` result; do not erase hashes or reset attempts to bypass it.
Never share database credentials or admin secrets in chat or logs.

The backfill performs chain reads and checked database metadata writes. It does
not submit transactions, create keys, change amounts or mint tokens. It always
pins the legacy `admin_v1` flow. Keep `WALLET_ENCRYPTION_KEY` unchanged.

## Switch the application together

Only after the backfill and candidate checks pass:

1. Update `deployed_addresses_testnet.json` and the contracts README deployment
   table with the candidate bundle. Preserve the old manifest for maintenance.
2. Update backend `src/config/soroban-contracts.ts` and frontend `lib/contracts.ts`
   to the same bundle. Select `point_owner_v1` for new work. Check Render's
   `SOROBAN_CONTRACTS` and `SOROBAN_COLLECTION_FLOW` overrides if present.
3. Update public docs and `/stats` to describe the actual point-signed flow. Keys
   remain custodial, registry trust is administered by the platform and the
   explicit contract admin override remains available. Payment amounts are still
   supplied by the backend until the separate on-chain payout change ships.
4. Deploy and verify the demo collector/operator flow. Follow the record,
   point-verification and payment transactions on testnet. Confirm older pinned
   collections still use their original contracts and flow.

Do not remove old role grants while old collections may need them. A rollback
changes defaults only for work that has not started; already pinned contexts
retain their contracts and flow. Preserve private point keys in the database.

## Check the tooling locally

```sh
python3 -m unittest discover -s scripts/tests -v
```

The tests cover lost deployment and initialization responses, repeat preparation,
legacy role preservation, preflight failures and tampered or changed journals.
