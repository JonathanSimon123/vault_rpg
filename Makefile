# Vault RPG Makefile
# A mnemonic vault with puzzle games

# Variables
BINARY_NAME=vault_rpg
CARGO=cargo
RUSTFLAGS=-C target-cpu=native

# Default target
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	@echo "🔨 Building Vault RPG..."
	$(CARGO) build

# Build in release mode
.PHONY: release
release:
	@echo "🚀 Building Vault RPG in release mode..."
	$(CARGO) build --release

# Run the project
.PHONY: run
run:
	@echo "🎮 Running Vault RPG..."
	$(CARGO) run -- menu

# Run in release mode
.PHONY: run-release
run-release: release
	@echo "🎮 Running Vault RPG (release mode)..."
	$(CARGO) run --release

# Clean build artifacts
.PHONY: clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	$(CARGO) clean

# Run tests
.PHONY: test
test:
	@echo "🧪 Running tests..."
	$(CARGO) test

# Run tests with output
.PHONY: test-verbose
test-verbose:
	@echo "🧪 Running tests (verbose)..."
	$(CARGO) test -- --nocapture

# Check code without building
.PHONY: check
check:
	@echo "🔍 Checking code..."
	$(CARGO) check

# Format code
.PHONY: fmt
fmt:
	@echo "✨ Formatting code..."
	$(CARGO) fmt

# Lint code
.PHONY: lint
lint:
	@echo "🔍 Linting code..."
	$(CARGO) clippy

# Lint with suggestions
.PHONY: lint-fix
lint-fix:
	@echo "🔧 Fixing linting issues..."
	$(CARGO) clippy --fix --allow-dirty

# Update dependencies
.PHONY: update
update:
	@echo "📦 Updating dependencies..."
	$(CARGO) update

# Install dependencies
.PHONY: deps
deps:
	@echo "📦 Installing dependencies..."
	$(CARGO) fetch

# Generate documentation
.PHONY: docs
docs:
	@echo "📚 Generating documentation..."
	$(CARGO) doc --open

# Generate documentation without opening
.PHONY: docs-build
docs-build:
	@echo "📚 Building documentation..."
	$(CARGO) doc

# Benchmark
.PHONY: bench
bench:
	@echo "⚡ Running benchmarks..."
	$(CARGO) bench

# Install binary to system
.PHONY: install
install: release
	@echo "📦 Installing Vault RPG to system..."
	$(CARGO) install --path .

# Uninstall binary from system
.PHONY: uninstall
uninstall:
	@echo "🗑️ Uninstalling Vault RPG..."
	$(CARGO) uninstall $(BINARY_NAME)

# Create a distribution package
.PHONY: package
package: release
	@echo "📦 Creating distribution package..."
	@mkdir -p dist
	@cp target/release/$(BINARY_NAME) dist/
	@cp README.md dist/
	@cp README_CN.md dist/
	@cp LICENSE dist/
	@echo "Package created in dist/ directory"

# Create a tar.gz package
.PHONY: dist
dist: package
	@echo "📦 Creating tar.gz distribution..."
	@cd dist && tar -czf ../$(BINARY_NAME)-$(shell date +%Y%m%d).tar.gz *
	@echo "Distribution package: $(BINARY_NAME)-$(shell date +%Y%m%d).tar.gz"

# Development setup
.PHONY: dev-setup
dev-setup: deps
	@echo "🛠️ Setting up development environment..."
	@echo "Installing development tools..."
	@rustup component add rustfmt clippy
	@echo "Development setup complete!"

# Security audit
.PHONY: audit
audit:
	@echo "🔒 Running security audit..."
	@cargo audit || echo "cargo-audit not installed. Install with: cargo install cargo-audit"

# Check for outdated dependencies
.PHONY: outdated
outdated:
	@echo "📅 Checking for outdated dependencies..."
	@cargo outdated || echo "cargo-outdated not installed. Install with: cargo install cargo-outdated"

# Run with environment variables
.PHONY: run-env
run-env:
	@echo "🎮 Running Vault RPG with environment variables..."
	@echo "Make sure to set VAULT_TOTP_SECRET, VAULT_TOTP_ACCOUNT, VAULT_TOTP_ISSUER"
	$(CARGO) run

