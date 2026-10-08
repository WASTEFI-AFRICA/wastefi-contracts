# Contributing to WasteFi Contracts

Thanks for your interest in contributing.

## Getting set up

Requires Rust 1.79.0 or later. The toolchain, `rustfmt`, `clippy` and the
`wasm32-unknown-unknown` target are pinned in
[`rust-toolchain.toml`](rust-toolchain.toml), so `rustup` provisions them on
first build.

```sh
make install    # add the wasm target and install soroban-cli
make setup      # register the testnet network with soroban-cli
make build
```

## Checks a pull request must pass

```sh
make check      # cargo fmt, clippy -D warnings, test, and a release wasm build
```

Note that `cargo test --workspace` does not currently compile against the
pinned `soroban-sdk` 21.7.7; see the testing section of the
[README](README.md#testing). If your change touches a crate's tests, say in the
pull request whether you were able to run them.

## Conventions

- Contracts are `no_std`. Keep them that way: the deployed wasm size is a direct
  cost on every call.
- Shared types, errors, events and access-control helpers live in
  `contracts/common`. Put anything used by two contracts there rather than
  duplicating it.
- Return a `Result` with a `#[contracterror]` variant for conditions a caller
  can act on. Reserve `panic!` for genuinely unreachable states, and prefer
  converting existing panics to errors when you touch that code.
- Use checked arithmetic on anything touching balances or weights. The release
  profile keeps `overflow-checks` on deliberately; do not turn it off to silence
  a warning.
- Emit an event for every state change an indexer or auditor would need to see.
- Document public entry points with rustdoc, including which role may call them.

## Tests

New behaviour needs tests. Unit tests live beside the code in each crate's
`src/test.rs`; cross-contract scenarios go in [`tests/`](tests). Cover the error
paths and the authorization checks, not just the happy path — most of what these
contracts need to get right is who is allowed to call what. See
[docs/WRITING_TESTS.md](docs/WRITING_TESTS.md).

## Changes that need extra care

Anything touching access control, payment amounts, token minting, or the release
profile in `Cargo.toml` changes the trust model or the fee profile. Explain the
reasoning in the pull request and expect a closer review.

## Reporting bugs

Open an issue with what you expected, what happened, and the smallest set of
steps that reproduces it. Include versions and, where relevant, logs or a
failing test. Search existing issues first.

Do not report security vulnerabilities as issues — see [SECURITY.md](SECURITY.md).

## Pull requests

1. Fork the repository and branch from `main`.
2. Make your change, with tests for anything that changes behaviour.
3. Run the checks listed above and make sure they pass.
4. Open a pull request describing what changed and why. Link the issue it
   closes, if there is one.

Keep a pull request to one logical change. A branch that reformats half the tree
alongside a bug fix is hard to review and harder to revert.

## Commit messages

Write messages in the imperative mood, explaining why rather than restating the
diff:

```
Reject payouts for unverified collections

process_payment accepted any transaction id, so a payout could be
recorded against a collection that was never verified. Check the
verified flag before writing the payment record.
```

[Conventional Commits](https://www.conventionalcommits.org/) prefixes
(`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`) are welcome but not
required.

## Code of conduct

Be straightforward and civil. We follow the
[Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/);
report unacceptable behaviour to conduct@wastefi.org.
