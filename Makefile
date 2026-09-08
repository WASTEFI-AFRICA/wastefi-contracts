.PHONY: build test clean install fmt lint optimize

# Build all contracts
build:
	cargo build --target wasm32-unknown-unknown --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install dependencies
install:
	rustup target add wasm32-unknown-unknown
	cargo install --locked soroban-cli

# Format code
fmt:
	cargo fmt --all

# Run linter
lint:
	cargo clippy --all-targets -- -D warnings

# Build optimized contracts
optimize: build
	@echo "Contracts built and optimized in target/wasm32-unknown-unknown/release/"

# Run all checks (fmt, lint, test, build)
check: fmt lint test build
	@echo "All checks passed!"

# Deploy to testnet (requires DEPLOYER_SECRET env var)
deploy-testnet:
	@echo "Deploying to Stellar testnet..."
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/waste_token.wasm \
		--source deployer \
		--network testnet

# Setup development environment
setup: install
	soroban network add testnet \
		--rpc-url https://soroban-testnet.stellar.org:443 \
		--network-passphrase "Test SDF Network ; September 2015"
	@echo "Development environment ready!"
