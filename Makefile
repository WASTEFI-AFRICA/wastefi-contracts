.PHONY: build test clean install fmt lint optimize check deploy-testnet smoke-test

# Build all contracts to wasm. scripts/build-wasm.sh explains why this is not a
# plain `cargo build`: the workspace root is itself a package, and building the
# contracts with the default crate types produces oversized wasm.
build:
	./scripts/build-wasm.sh

# Run the test suite
test:
	cargo test --workspace

# Remove build artifacts
clean:
	cargo clean

# Install the wasm target and the Stellar CLI
install:
	rustup target add wasm32v1-none
	cargo install --locked stellar-cli

# Format all sources
fmt:
	cargo fmt --all

# Lint, treating warnings as errors (as CI does)
lint:
	cargo clippy --workspace --all-targets -- -D warnings

# Build and report the size of each contract; wasm size drives deployment fees
optimize: build

# Everything CI checks, in the same order
check:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	cargo check --workspace
	cargo test --workspace
	./scripts/build-wasm.sh
	@echo "All checks passed."

# Build, deploy, initialize and wire all contracts on testnet (see scripts/deploy-testnet.sh)
deploy-testnet:
	./scripts/deploy-testnet.sh

# Drive one delivery through the deployed testnet contracts
smoke-test:
	./scripts/smoke-test.sh
