# WasteFi — Contracts

Soroban smart contracts for WasteFi, a platform that pays waste collectors in
emerging markets for verified recyclable material drop-offs on Stellar.

A collector registers once and delivers sorted material to a collection point.
The delivery is recorded on-chain, verified, and then paid out in reward tokens,
with the collector's reputation score tracking their verified history. The
contracts are at an early stage: the intended trust model puts verification in
the hands of collection points, but as deployed it is the platform admin that
verifies deliveries and sets payout amounts. See
[how a collection becomes a payment](#how-a-collection-becomes-a-payment) for
what is actually wired up today.

For the HTTP API and indexer that sit in front of these contracts, see
[wastefi-backend](https://github.com/WASTEFI-AFRICA/wastefi-backend).

## Contracts

| Crate | Responsibility |
| --- | --- |
| `common` | Shared types, errors, events, access control, and validation used by every other crate |
| `collector_registry` | Collector identity, registration status, and profile data |
| `collection_point` | Registry of collection points; identity only, not yet the authority that verifies |
| `waste_transaction` | Records a delivery and moves it through pending, verified, and completed |
| `material_pricing` | Per-material price oracle, written by an operator role |
| `payment_distribution` | Records a payout against a transaction id; the amount is supplied by the caller |
| `reputation` | Integer reputation scoring derived from verified delivery history |
| `waste_token` | The reward token: mint, burn, transfer and balances. Allowance storage exists but no `approve`/`transfer_from` entry points are exposed yet |

`common` is a library rather than a deployed contract. The other seven each
build to their own wasm.

## How a collection becomes a payment

The contracts are split by concern, but the current wiring routes almost every
state change through the platform admin rather than through the collection point
that physically received the material:

1. `collector_registry::register` records a collector.
2. `waste_transaction::record_collection` (or `record_with_price_lookup`, which
   additionally calls `collector_registry::is_active` to reject inactive
   collectors) records a delivery as pending.
3. `waste_transaction::verify_transaction` marks the delivery verified and
   completed. **This call requires the contract admin**, not the collection
   point. Collection-point identity is registered in `collection_point` but is
   not yet what authorizes a verification.
4. `payment_distribution::process_payment` records what is owed. It also
   requires admin, and it takes `amount` as a caller-supplied argument — it does
   not read the verified transaction or price it against `material_pricing`.
5. `waste_token` mints against that record, gated on the minter role.
6. `reputation::update_score` folds the result into the collector's score.

Two cross-contract links are stubbed rather than implemented:
`record_with_price_lookup` resolves the `material_pricing` address but still
carries a `TODO` instead of calling `get_price`, and `verify_transaction` has a
`TODO` where it should notify `reputation`. Both currently require the caller to
pass the value in or the admin to make a second call.

The practical consequence is that the admin key is trusted for verification,
payout amounts, and minting. Reducing that trust — deriving amounts on-chain,
authorizing verification from `collection_point`, and splitting the admin into a
multisig — is the main item in
[docs/SECURITY_ROADMAP.md](docs/SECURITY_ROADMAP.md).

## Release profile

The workspace `Cargo.toml` sets a non-default `[profile.release]`. Soroban
charges per byte of deployed wasm and per CPU instruction executed, so profile
choices here translate directly into what every call costs:

| Setting | Value | Why |
| --- | --- | --- |
| `opt-level` | `"z"` | Optimize for size; wasm size drives upload and storage fees. |
| `lto` | `true` | Whole-program optimization, trimming dead code across crates. |
| `codegen-units` | `1` | Gives the optimizer the whole crate at once, trading build time for a smaller artifact. |
| `panic` | `"abort"` | Drops unwinding tables; Soroban traps on panic and cannot unwind across the host boundary. |
| `strip` | `"symbols"` | Symbol and debug info is pure size cost on-chain. |
| `debug` | `0` | Same rationale as `strip`. |
| `overflow-checks` | `true` | Kept **on**, against the Rust release default. These contracts move token balances, and a silently wrapped `i128` is far worse than a checked add. |

A second profile, `release-with-logs`, inherits from `release` but re-enables
debug assertions for diagnosing a build without changing the release settings.

Relaxing `opt-level`, `lto`, or `strip` grows the deployed wasm and raises fees.
Turning `overflow-checks` off would let balance arithmetic wrap silently.

## Build

Requires Rust 1.79.0 or later. The toolchain, components, and wasm target are
pinned in [`rust-toolchain.toml`](rust-toolchain.toml), so `rustup` provisions
them on first build.

```sh
make build          # cargo build --target wasm32-unknown-unknown --release
make test           # cargo test --workspace
make lint           # cargo clippy --all-targets -- -D warnings
make fmt            # cargo fmt --all
make check          # fmt, lint, test, build
```

Built artifacts land in `target/wasm32-unknown-unknown/release/`.

## Testing

```sh
cargo test --workspace          # all unit tests
cargo test -p payment_distribution    # one crate
cargo test --test integration_e2e     # end-to-end scenarios
cargo test --test stress_tests        # high-volume scenarios
```

The suite is 146 unit tests across the eight crates plus 58 integration tests
in [`tests/`](tests). Coverage is uneven: `common` carries 54 of the unit tests,
while `waste_transaction` has none of its own and is exercised only through the
integration tests.

**The test suite does not currently compile.** The wasm release build is fine,
but building for the host target pulls in `soroban-sdk`'s `testutils`, and
`soroban-env-host` 21.2.1 fails against the `ed25519-dalek` version that now
resolves. The CI test job is a stub that echoes a message instead of running
`cargo test`, so CI reports green while executing none of these tests. Fixing
this means moving off `soroban-sdk` 21.7.7, which is many major versions behind;
see the open Dependabot pull requests. Until then, treat the test counts above
as tests that exist, not tests that pass.

CI ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs
`cargo fmt --check`, `cargo clippy -D warnings`, `cargo check`, a release wasm
build, and `cargo audit` on every push and pull request.

## Deployment

Network configuration lives in [`config/`](config); see
[config/README.md](config/README.md) for the shape of those files.

```sh
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

soroban keys generate deployer --network testnet
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"

./scripts/deploy.sh testnet config/testnet.json
```

Full instructions, including initialization order and the operator role grants
each contract needs, are in [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md). Day-to-day
operational procedures are in [docs/OPERATIONS.md](docs/OPERATIONS.md).

### Deployed addresses

Current testnet addresses are in
[`deployed_addresses_testnet.json`](deployed_addresses_testnet.json). There is
no mainnet deployment; see the audit status below.

## Security

These contracts have **not** been audited and are **not** deployed to mainnet.
Do not use them to hold real value in their present state.

Known gaps that must close before a mainnet deployment are tracked in
[docs/SECURITY_ROADMAP.md](docs/SECURITY_ROADMAP.md). It lists, in its own
priority order: no double-payment prevention in `payment_distribution`, no
supply cap in `waste_token`, a single admin key where a multisig belongs,
incomplete collector-status validation, and weak rate limiting.

- [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md) — who these contracts defend against and what the admin can do
- [docs/SECURITY_CONSIDERATIONS.md](docs/SECURITY_CONSIDERATIONS.md) — per-contract analysis
- [docs/AUDIT_SCOPE.md](docs/AUDIT_SCOPE.md) — what an external audit should cover
- [docs/INCIDENT_RESPONSE.md](docs/INCIDENT_RESPONSE.md) — emergency procedures

To report a vulnerability, see [SECURITY.md](SECURITY.md). Please do not open a
public issue for one.

## Documentation

- [docs/DEVELOPER.md](docs/DEVELOPER.md) — architecture and internals
- [docs/API.md](docs/API.md) — every contract entry point, argument, and error
- [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) — local environment setup
- [docs/WRITING_TESTS.md](docs/WRITING_TESTS.md) — how to write tests for this workspace
- [docs/TESTING.md](docs/TESTING.md) — test strategy and coverage
- [docs/EVENT_INDEXING.md](docs/EVENT_INDEXING.md) — events emitted, for indexer authors
- [docs/GAS_OPTIMIZATION.md](docs/GAS_OPTIMIZATION.md) — storage and fee notes
- [docs/UPGRADE_GUIDE.md](docs/UPGRADE_GUIDE.md) — contract upgrade and migration
- [docs/USER_GUIDE.md](docs/USER_GUIDE.md) — collector and collection-point walkthrough

## Related repositories

- [wastefi-backend](https://github.com/WASTEFI-AFRICA/wastefi-backend) — REST API, indexer, and mobile money integration
- [wastefi-frontend](https://github.com/WASTEFI-AFRICA/wastefi-frontend) — collector and operator progressive web app
- [wastefi-docs](https://github.com/WASTEFI-AFRICA/wastefi-docs) — platform documentation site

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Pull requests need `cargo fmt`,
`cargo clippy -D warnings`, and `cargo test --workspace` to pass.

## License

MIT. See [LICENSE](LICENSE).
