<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/wastefi-logo-dark.svg">
    <img src="assets/wastefi-logo.svg" alt="WasteFi" height="64">
  </picture>
</p>

# WasteFi — Contracts

Soroban smart contracts for WasteFi, a platform that pays waste collectors in
emerging markets for verified recyclable material drop-offs on Stellar.

A collector registers once and delivers sorted material to a collection point.
The delivery is recorded on-chain, verified, and then paid out in reward tokens,
with the collector's reputation score tracking their verified history. The
contracts are at an early stage. New work can be verified by the collection
point's key through a checked registry. The API holds custodial signing keys,
the platform controls registry trust and payments, and an explicit admin
verification override remains. See
[how a collection becomes a payment](#how-a-collection-becomes-a-payment).

For the HTTP API and indexer that sit in front of these contracts, see
[wastefi-backend](https://github.com/WASTEFI-AFRICA/wastefi-backend).

## Try it

The bundle under [Deployment](#deployment) supports point-signed verification on
Stellar testnet. The app's public `/stats` page reports the API's configured
verification mode for new work; older collections retain their saved contracts
and flow. Sign in with a demo account on
[the frontend](https://wastefi-frontend.vercel.app/login), approve a collection,
and follow the record, verify and payment transactions it links to on
stellar.expert. To exercise them directly, `scripts/smoke-test.sh` drives one
delivery through all seven contracts from the command line.

## Contracts

| Crate | Responsibility |
| --- | --- |
| `common` | Shared types, errors, events, access control, and validation used by every other crate |
| `collector_registry` | Collector identity, registration status, and profile data |
| `collection_point` | Registry of collection point owners, verification status and accepted materials; the point-verification path checks it |
| `waste_transaction` | Records a delivery and moves it through pending, verified, and completed |
| `material_pricing` | Per-material price oracle, written by an operator role |
| `payment_distribution` | Records one payout per transaction and mints it in WASTE when completed; the collector and amount are read from the verified transaction, with a logged admin override |
| `reputation` | Integer reputation scoring derived from verified delivery history |
| `waste_token` | The reward token: capped mint, burn, transfer, and approve/transfer_from allowances |

`common` is a library rather than a deployed contract. The other seven each
build to their own wasm.

## How a collection becomes a payment

With `point_owner_v1` selected in the API, new approvals follow this flow:

1. `collector_registry::register` records a collector. The admin activates it.
2. The API keeps an encrypted point wallet, registers it in `collection_point`
   with pseudonymous metadata, and has the registry admin verify it after checking
   the database point's activation, verification and accepted materials.
3. `waste_transaction::record_with_price_lookup` records a pending delivery at
   the price from `material_pricing`. The collector signs and must be active.
4. `waste_transaction::verify_transaction_by_point` requires the recorded point
   owner's signature, a verified registry entry accepting the material, and both
   contracts unpaused. It marks the record completed and updates reputation.
5. `payment_distribution::process_verified_payment` records one payment per
   transaction. It requires admin to trigger it, but the collector and amount are
   read on-chain from the verified transaction record, which the contract checks
   is verified. The admin override `process_payment` can still pay a recipient and
   amount the admin chooses, and emits a `pay_ovr` event each time.
6. `payment_distribution::update_payment_status(Completed)` mints the payment
   through `waste_token::mint_from`. The payment contract has a minter role.

Saved `admin_v1` collections retain their original contracts and admin
verification. New point-authorization failures never fall back to admin.
The platform still holds signing keys and controls payment amounts. The explicit
admin verification override remains, including while paused and for old records
whose point key was never retained. This is not independent point custody.

## Point verification

The updated transaction contract provides `waste_transaction::verify_transaction_by_point`.
It requires the signature of the `collection_point` address stored in the
transaction, looks up that owner in the configured `collection_point` registry,
and requires a verified point that accepts the recorded material. The delivery
must still be pending, and both contracts must be unpaused. Missing registry
configuration or a failed check rejects the call without changing the delivery
or reputation. The deploy script wires the registry through the admin-only
`set_collection_point_contract` method.

This path uses the same reputation update as individual admin verification.
Both individual methods keep the existing `tx_ver` event and add an actor event:
topics `(verify_by, transaction_id)`, data `verifier: Address`. The legacy batch
method is unchanged and does not emit the actor event.

`verify_transaction` remains an explicit admin override, including while paused
and for old records whose point key was never retained. The new point method
never falls back to that override. This preserves the admin trust boundary;
it does not make verification independent of the platform admin.

The updated registry exposes `find_point_by_owner -> Option<u64>` so the API
can distinguish an absent registration from a failed RPC read. It is public and
read-only. The new bundle reuses the original token, collector registry, pricing
and reputation contracts. Its registry, transaction contract and payment map are
new. Older role grants stay in place so pinned collections remain resumable.

Use the [point-verification rollout guide](docs/POINT_VERIFICATION_ROLLOUT.md) to
prepare the three replacement contracts while keeping the current token,
balances and legacy roles. Its restartable script writes a candidate manifest;
it leaves the active address file and hosted application unchanged.

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
the root emits no contract wasm at all. The build script reports the size of each artifact.

## Testing

```sh
cargo test --workspace                 # everything
cargo test -p payment_distribution     # one crate
cargo test --test integration_test     # cross-contract scenarios
```

Cross-contract scenarios in [`tests/integration_test.rs`](tests/integration_test.rs)
exercise registration, collection, pricing, payments and reputation.
[`tests/point_verification.rs`](tests/point_verification.rs) adds owner-only
verification, foreign and missing signatures, unregistered and unverified points,
registry configuration, pauses, invalid delivery states, material restrictions,
reputation rollback, duplicate verification, admin recovery and WASTE payment.
These tests use real contract implementations inside the Soroban host test
environment. They do not demonstrate deployment or backend key custody.

Coverage is uneven, and no coverage figure is claimed.

CI ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs
`cargo fmt --check`, `cargo clippy -D warnings`, `cargo check`, `cargo test
--workspace`, a release wasm build that fails unless all seven contracts
produce an artifact, and `cargo audit` on every push and pull request.

## Deployment

The contracts are deployed on Stellar testnet. There is no mainnet deployment; see
the audit status below.

| Contract | Testnet address |
| --- | --- |
| `waste_token` | [`CDOK6OEYIINSA2KP23UU72BSNST7PFNVD67DI2PDELCZIQYUV7XK7SH6`](https://stellar.expert/explorer/testnet/contract/CDOK6OEYIINSA2KP23UU72BSNST7PFNVD67DI2PDELCZIQYUV7XK7SH6) |
| `collector_registry` | [`CCP4QOY57OKA6DNRU2XG5AD5B2YSVNSUG7BRVWR5XJRDMGY62VQ7JHII`](https://stellar.expert/explorer/testnet/contract/CCP4QOY57OKA6DNRU2XG5AD5B2YSVNSUG7BRVWR5XJRDMGY62VQ7JHII) |
| `collection_point` | [`CADEZ64CHYORQLRKD3JLV6QLW7IHJAUWEAAOPS6K2DNMQSXZCSPLGMHW`](https://stellar.expert/explorer/testnet/contract/CADEZ64CHYORQLRKD3JLV6QLW7IHJAUWEAAOPS6K2DNMQSXZCSPLGMHW) |
| `material_pricing` | [`CA6BP7PC7OPZPT4MFHIJAPXB237BVP72GVNVPI67GPH3QYYBCJDN2Z5G`](https://stellar.expert/explorer/testnet/contract/CA6BP7PC7OPZPT4MFHIJAPXB237BVP72GVNVPI67GPH3QYYBCJDN2Z5G) |
| `reputation` | [`CCXDYT3RIM2D356CYUZJ7SVD5PRFUVNJDZDAKSI5WITMMW5QMXZHIB5A`](https://stellar.expert/explorer/testnet/contract/CCXDYT3RIM2D356CYUZJ7SVD5PRFUVNJDZDAKSI5WITMMW5QMXZHIB5A) |
| `waste_transaction` | [`CB77D77P7RLXHPXNCZG4PJE6OVAXWYWAY2KBDX2SO76FMSBL7XOTYDIT`](https://stellar.expert/explorer/testnet/contract/CB77D77P7RLXHPXNCZG4PJE6OVAXWYWAY2KBDX2SO76FMSBL7XOTYDIT) |
| `payment_distribution` | [`CBUDOQMESUCX3NRTLCS6TRAHPKRZTVBPUTNUBLYRGFPQ76LTQ7N5RLFG`](https://stellar.expert/explorer/testnet/contract/CBUDOQMESUCX3NRTLCS6TRAHPKRZTVBPUTNUBLYRGFPQ76LTQ7N5RLFG) |

Payment code updated 2026-11-09 from `3ca9b1d`, bound once to the existing
transaction contract so payouts are derived on-chain. The previous payment contract
keeps its minter role so collections already in progress can finish; it is archived
with the rest of the previous bundle in
[`deployments/testnet-2026-11-08.json`](deployments/testnet-2026-11-08.json). The
registry and transaction code date from 2026-11-08 (`edbbec3`), and the token,
collector registry, pricing and reputation retain their 2026-10-28 deployment. Prepared by
[`GBPLV2ID...3Y4L`](https://stellar.expert/explorer/testnet/account/GBPLV2IDOG7E2UQEDHTN2ZZA3OGUECFHM5F54S3F65QEUR5EATJA3Y4L),
which is also the admin of every contract. The addresses are recorded in
[`deployed_addresses_testnet.json`](deployed_addresses_testnet.json). Testnet is
reset periodically, so these will stop resolving eventually; redeploy with the
script below.

### Fresh deployment

Requires the [Stellar CLI](https://developers.stellar.org/docs/tools/cli) (`make install`
installs it).

```sh
make deploy-testnet     # new token and seven new contracts; not an app migration
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
the explicit admin payment override and token `mint`, custodial point keys and the explicit admin
verification override, incomplete collector-status validation,
and weak rate limiting.

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
