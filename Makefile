.PHONY: build test clean install fmt lint optimize check setup deploy-testnet

# The workspace root is itself a package (it holds the integration tests), so
# cargo would otherwise build only that package and emit no contract wasm.
# Every target below is explicit about --workspace for this reason.

# Build all contracts to wasm
build:
	cargo build --workspace --target wasm32-unknown-unknown --release

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

# Report the size of each built contract; wasm size drives deployment fees
optimize: build
	@echo "Built contracts:"
	@ls -1 target/wasm32-unknown-unknown/release/*.wasm | while read f; do \
		printf "  %-28s %7.1f KB\n" "$$(basename $$f)" "$$(echo "scale=1; $$(stat -c%s $$f)/1024" | bc)"; \
	done

# Everything CI checks, in the same order
check:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	cargo check --workspace
	cargo build --workspace --target wasm32-unknown-unknown --release
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
