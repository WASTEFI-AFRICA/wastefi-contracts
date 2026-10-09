.PHONY: build test clean install fmt lint optimize check setup deploy-testnet

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

# Install the wasm target and the Soroban CLI
install:
	rustup target add wasm32-unknown-unknown
	cargo install --locked soroban-cli

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
	./scripts/build-wasm.sh
	@echo "All checks passed."

# Register the Soroban testnet with the local CLI
setup: install
	soroban network add testnet \
		--rpc-url https://soroban-testnet.stellar.org:443 \
		--network-passphrase "Test SDF Network ; September 2015"
	@echo "Testnet registered."

# Deploy to testnet; see docs/DEPLOYMENT.md for the full procedure
deploy-testnet: build
	./scripts/deploy.sh testnet config/testnet.json
