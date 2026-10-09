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
| `waste_token` | The reward token: capped mint, burn, transfer, and approve/transfer_from allowances |

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

Requires Rust 1.84.0 or later. The toolchain, components, and wasm target are
pinned in [`rust-toolchain.toml`](rust-toolchain.toml), so `rustup` provisions
them on first build.

```sh
make build          # scripts/build-wasm.sh: one cdylib wasm per contract
make test           # cargo test --workspace
make lint           # cargo clippy --all-targets -- -D warnings
make fmt            # cargo fmt --all
make check          # fmt, lint, test, build
```

Built artifacts land in `target/wasm32v1-none/release/`.

Always build wasm through `make build` or `scripts/build-wasm.sh`, not a bare
`cargo build`. Two things go wrong otherwise. The contracts must target
`wasm32v1-none`: since Rust 1.82, `wasm32-unknown-unknown` emits WebAssembly
features that the Soroban VM rejects, so the build succeeds but the network
refuses to deploy the result. And `Cargo.toml` lists both `lib` (so the
integration tests can import the contracts) and `cdylib`, which makes a default
build noticeably larger; the script passes `--crate-type cdylib` per contract, as
the Stellar CLI does. The workspace root is itself a package, so a bare build at
the root emits no contract wasm at all. Current sizes are 18 to 39 KB per contract.

## Testing

```sh
cargo test --workspace                 # everything
cargo test -p payment_distribution     # one crate
cargo test --test integration_test     # cross-contract scenarios
```

194 tests, all passing: 166 unit tests across the crates and 28 integration
tests in [`tests/integration_test.rs`](tests/integration_test.rs) that deploy all
seven contracts into one test environment and drive them through registration,
collection, pricing, payment, reputation, pausing, and collector-status gating.

Coverage is uneven. `waste_transaction`, the most logic-heavy contract, has no
unit tests of its own and is exercised only through the integration tests. There
is no coverage measurement, so no coverage figure is claimed.

CI ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs
`cargo fmt --check`, `cargo clippy -D warnings`, `cargo check`, `cargo test
--workspace`, a release wasm build that fails unless all seven contracts
produce an artifact, and `cargo audit` on every push and pull request.

## Deployment

The contracts are deployed on Stellar testnet. There is no mainnet deployment; see
the audit status below.

| Contract | Testnet address |
| --- | --- |
| `waste_token` | [`CBSHEPK3FDF4E4A3NE25S6M4RDTJOJZGQVDG7PDLM4R2YLLBJMYTHDCS`](https://stellar.expert/explorer/testnet/contract/CBSHEPK3FDF4E4A3NE25S6M4RDTJOJZGQVDG7PDLM4R2YLLBJMYTHDCS) |
| `collector_registry` | [`CC6OULJTBVRE3TJBEXG5TAUIFXDF2JAARMSK6EKDCYK6FPQFCRUUKHJE`](https://stellar.expert/explorer/testnet/contract/CC6OULJTBVRE3TJBEXG5TAUIFXDF2JAARMSK6EKDCYK6FPQFCRUUKHJE) |
| `collection_point` | [`CDXRC6LFSXMSIHU3NXEOQE4OM7BA45NTYJS3LUSNEX7Q54LNUSKKPL33`](https://stellar.expert/explorer/testnet/contract/CDXRC6LFSXMSIHU3NXEOQE4OM7BA45NTYJS3LUSNEX7Q54LNUSKKPL33) |
| `material_pricing` | [`CBDQDXBEG7V3URBZR5HAOZF4ESICV3UZ5WDEO6NJQ5E2QLKPBEDVW45D`](https://stellar.expert/explorer/testnet/contract/CBDQDXBEG7V3URBZR5HAOZF4ESICV3UZ5WDEO6NJQ5E2QLKPBEDVW45D) |
| `reputation` | [`CARIL3VA74YS6JYA6P4D4MDAW3GFMSMWJ4JUMTM3RZWDKOTUU5ZIVWJG`](https://stellar.expert/explorer/testnet/contract/CARIL3VA74YS6JYA6P4D4MDAW3GFMSMWJ4JUMTM3RZWDKOTUU5ZIVWJG) |
| `waste_transaction` | [`CDB2G3EVSRIG5UKRTEKGFT3CM6VJYAFVAQAV2DLOTC6U3UISSLB4LV2X`](https://stellar.expert/explorer/testnet/contract/CDB2G3EVSRIG5UKRTEKGFT3CM6VJYAFVAQAV2DLOTC6U3UISSLB4LV2X) |
| `payment_distribution` | [`CDEXPCJ7IR3LL7GW3QWETNOTISUYWUCKPUJ2JPKPKYTMKDTVXVWJJYKN`](https://stellar.expert/explorer/testnet/contract/CDEXPCJ7IR3LL7GW3QWETNOTISUYWUCKPUJ2JPKPKYTMKDTVXVWJJYKN) |

Deployed 2026-10-09 from `a460df1+uncommitted` by
[`GBPLV2ID...3Y4L`](https://stellar.expert/explorer/testnet/account/GBPLV2IDOG7E2UQEDHTN2ZZA3OGUECFHM5F54S3F65QEUR5EATJA3Y4L),
which is also the admin of every contract. The addresses are recorded in
[`deployed_addresses_testnet.json`](deployed_addresses_testnet.json). Testnet is
reset periodically, so these will stop resolving eventually; redeploy with the
script below.

### Deploy it yourself

Requires the [Stellar CLI](https://developers.stellar.org/docs/tools/cli) (`make install`
installs it).

```sh
make deploy-testnet     # build, deploy, initialize and wire all seven contracts
make smoke-test         # drive a delivery through the deployed contracts
```

`scripts/deploy-testnet.sh` creates and funds a testnet identity named
`wastefi-deployer` on first run (its key stays in the Stellar CLI's own store and
is never written into the repository), then deploys the seven contracts,
initializes them with that identity as admin, points `waste_transaction` at the
pricing, reputation and registry contracts, and records the addresses.

`scripts/smoke-test.sh` then exercises the deployed contracts end to end with
freshly generated throwaway accounts: it registers and activates a collector,
registers and verifies a collection point, records a collection, checks that
recording one in an active collector's name without their signature is rejected,
verifies it, checks the stored payout, records the payment, mints the reward, and
checks the collector's balance. It exits non-zero on the first failure.

Further background, including the role each contract needs, is in
[docs/DEPLOYMENT.md](docs/DEPLOYMENT.md); day-to-day operations are in
[docs/OPERATIONS.md](docs/OPERATIONS.md).

## Security

These contracts have **not** been audited and are **not** deployed to mainnet.
Do not use them to hold real value in their present state.

Known gaps that must close before a mainnet deployment are tracked in
[docs/SECURITY_ROADMAP.md](docs/SECURITY_ROADMAP.md). The token supply cap is
now enforced in `mint` and a completed or failed payment can no longer be
modified. Still open: a single admin key where a multisig belongs, payout
amounts supplied by the caller, verification gated on the admin rather than
the collection point, incomplete collector-status validation, and weak rate
limiting.

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
