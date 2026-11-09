# Deriving payouts on-chain on testnet

`payment_distribution` can now read what a delivery is worth from the verified
transaction instead of trusting an amount from its caller. This rollout deploys a
new payment contract with that behavior. It changes one contract. The WASTE token,
balances, registry, pricing, reputation, point registry and transaction contract all
stay in place.

## What changes

- `process_verified_payment(transaction_id)` reads the transaction from the bound
  transaction contract. It requires the transaction to be verified and completed,
  pays the collector on the record, and pays the record's `total_amount`. The caller
  chooses neither the recipient nor the amount. `batch_process_verified` does the
  same for several transactions and aborts if any one is unverified, unknown or
  already paid.
- `set_transaction_contract` binds that transaction contract once. It cannot be
  changed later, so the payment contract cannot be pointed at records that were
  never verified for it.
- `process_payment` and `batch_process` remain as an **explicit admin override**:
  they pay a recipient and amount the admin supplies, check nothing against the
  transaction, and emit a `pay_ovr` event for every payment so an override is
  visible on-chain. The WASTE token's own admin `mint` is separate and unchanged.
  Deriving the normal path removes mistakes and makes it verifiable, but the admin
  key is still trusted.
- **Known limit:** one payment per transaction is enforced per payment contract.
  Transactions the previous payment contract already paid are not in the new
  contract's map, so an admin calling `process_verified_payment` for one of those ids
  by hand would pay it a second time. The backend only pays collections that start
  after the switch and never does this, but the contracts do not enforce the rule
  across the two payment contracts.

Collections that were already pinned to the previous payment contract keep using it
with its caller-supplied amounts. Only collections approved after the application
switches use the new contract.

## Prepare the candidate

Requires Python 3 on Linux or macOS, Stellar CLI 27, the existing
`wastefi-deployer` identity and the current testnet manifest. No secret key is
passed to the script. It uses the same journal, deterministic IDs and checks as the
[point verification rollout](POINT_VERIFICATION_ROLLOUT.md). Do not run the
seven-contract deploy script: that would create a different token with different
balances.

```sh
scripts/build-wasm.sh payment_distribution
python3 scripts/rollout-derived-payments-testnet.py
python3 scripts/rollout-derived-payments-testnet.py --apply
```

The first Python command only prints the plan. The second checks the existing
admins, pause flags, payment token and minter role before writing. It saves a random
salt and the deterministic contract ID before deploying anything, checks the deployed
WASM hash, initializes the contract, binds the existing transaction contract, and
adds the new contract as a token minter. The old payment contract keeps its minter
role so saved collections can finish.

The output is the ignored `target/derived-payments-testnet.json`. It cannot overwrite
the active manifest. Retain it and rerun the same command after an interruption;
never delete it to retry. A changed WASM, a changed base bundle, a salt that no
longer matches or a different existing transaction binding stops the rollout for
review.

## Switch the application

Only after the candidate checks pass:

1. Update `deployed_addresses_testnet.json`, keep the previous manifest under
   `deployments/`, and update the contracts README table.
2. Update the backend's default bundle, so collections approved from then on pin the
   new payment contract and the derived call. Update the frontend's contract list.
3. Deploy the backend first, then the frontend and docs. Approve one collection as
   the demo operator and follow its record, verification and payment transactions.
   Confirm the payment was created with `process_verified_payment`.
4. Collections pinned earlier must still finish on their original contracts.

## Check the tooling locally

```sh
python3 -m unittest discover -s scripts/tests -v
cargo test --workspace
```
