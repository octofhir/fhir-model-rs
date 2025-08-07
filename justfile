# Justfile for octofhir-fhir-model
# Common commands for development, testing, and release preparation

# Default recipe - show available commands
default:
    @just --list

# Format code using rustfmt
fmt:
    cargo fmt --all

# Check code formatting without making changes
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with fixes applied automatically
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run tests
test:
    cargo test

# Run tests with all features enabled
test-all:
    cargo test --all-features

# Run tests in verbose mode
test-verbose:
    cargo test --verbose --all-features

# Generate documentation
doc:
    cargo doc --no-deps --all-features

# Open generated documentation in browser
doc-open:
    cargo doc --no-deps --all-features --open

# Clean build artifacts
clean:
    cargo clean

# Check if the project compiles without building
check:
    cargo check --all-targets --all-features

# Run all pre-commit checks (format, lint, test)
pre-commit: fmt-check lint test-all

# Prepare for release - run all checks and build
prepare-release: clean fmt-check lint test-all build-release doc
    @echo "✅ All release preparation checks passed!"
    @echo "📦 Project is ready for release"

# Run a dry-run publish to check if everything is ready for crates.io
publish-dry-run:
    cargo publish --dry-run

# Show project information
info:
    @echo "📋 Project Information:"
    @echo "Name: $(grep '^name = ' Cargo.toml | head -1 | sed 's/name = \"\(.*\)\"/\1/')"
    @echo "Version: $(grep '^version = ' Cargo.toml | head -1 | sed 's/version = \"\(.*\)\"/\1/')"
    @echo "Edition: $(grep '^edition = ' Cargo.toml | head -1 | sed 's/edition = \"\(.*\)\"/\1/')"

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Run security audit
audit:
    cargo audit

# Install development tools
install-tools:
    cargo install cargo-outdated cargo-audit

# Run comprehensive CI-like checks locally
ci: clean fmt-check lint test-all build-release doc
    @echo "🎉 All CI checks passed locally!"

# Quick development cycle - format, check, and test
dev: fmt check test
    @echo "🚀 Development cycle complete!"
