# Testing

What tests exist, how to run them, and what they do not cover. For how to write
a new test, see [WRITING_TESTS.md](WRITING_TESTS.md).

## Running

```sh
cargo test --workspace                 # everything
cargo test -p waste_token              # one crate
cargo test -p waste_token mint         # tests whose name contains "mint"
cargo test --test integration_test     # cross-contract scenarios only
```

All tests run on the host against Soroban's in-process test environment. Nothing
here touches a network, and the whole suite finishes in a few seconds.

## What exists

Unit tests sit beside the code they test: `src/test.rs` in each contract crate,
and inline `mod tests` blocks in `common`.

| Crate | Tests | Notes |
| --- | --- | --- |
| `common` | 54 | Access control, anti-fraud, emergency controls, upgrade bookkeeping, validation, payout arithmetic |
| `waste_token` | 35 | Mint, burn, transfer, pause, supply cap, allowances, event emission |
| `payment_distribution` | 22 | Payment creation, batching, status transitions, pause |
| `reputation` | 16 | Score updates and statistics |
| `material_pricing` | 15 | Price reads and updates |
| `collector_registry` | 12 | Registration, status changes, queries |
| `collection_point` | 12 | Registration, verification, queries |
| `waste_transaction` | 0 | See gaps below |

`tests/integration_test.rs` holds 28 tests. Each builds a fixture that deploys
and initializes all seven contracts in one environment, then drives them through
real flows: registering a collector and a collection point, recording
collections, payments and reputation, pausing every contract, and checking that
a pending, suspended or banned collector cannot submit a transaction.

## What the tests do not cover

- **`waste_transaction` has no unit tests.** It is the most logic-heavy contract
  and is covered only through the integration tests.
- **Authorization is mostly mocked.** The unit tests call `mock_all_auths()` 93
  times and the integration fixture calls it too, so they check contract logic,
  not that a missing signature is rejected. Where a contract checks the admin explicitly, there are
  `should_panic` tests for it; there is no systematic test that every
  privileged entry point rejects an unauthorized caller.
- **No coverage measurement.** No coverage figure is claimed because none is
  measured.
- **No fuzzing, property-based, or load tests.** There is no `proptest` or
  similar dependency, and nothing exercises behaviour at high transaction volume.
- **No testnet tests.** The deployed contracts are not exercised by any test.

## Bugs the tests have caught

Worth knowing when deciding how much to trust the rest:

- Six persistent-storage writes in `collector_registry`, `collection_point`,
  `waste_transaction` and `waste_token` extended the TTL of a key that was never
  written. That makes `register` and `register_point` panic, and would on-chain
  too. The unit tests were not running in CI, so this went unnoticed.
- Payouts were computed as `(grams / 1000) * price`, truncating to whole
  kilograms, so a 500g delivery paid nothing.
- `mint` did not compile (an undefined variable), and the supply cap and
  `transfer_from` were lost in a merge.

## CI

[`.github/workflows/ci.yml`](../.github/workflows/ci.yml) runs
`cargo test --workspace` on every push and pull request, alongside formatting,
clippy with warnings denied, a wasm build that fails unless all seven contracts
produce an artifact, and `cargo audit`.
