#!/bin/bash
# Build and test verification script for hashing crate

set -e

echo "================================"
echo "Hashing Crate Verification"
echo "================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
        exit 1
    fi
}

print_info() {
    echo -e "${YELLOW}→${NC} $1"
}

# Check Rust installation
print_info "Checking Rust installation..."
if command -v rustc &> /dev/null; then
    rustc --version
    cargo --version
    print_status 0 "Rust is installed"
else
    print_status 1 "Rust is not installed. Please install from https://rustup.rs"
fi

echo ""

# Format check
print_info "Checking code formatting..."
cargo fmt -- --check
print_status $? "Code formatting"

echo ""

# Clippy check
print_info "Running Clippy lints..."
cargo clippy --all-targets --all-features -- -D warnings
print_status $? "Clippy checks"

echo ""

# Build library
print_info "Building library..."
cargo build --lib
print_status $? "Library build"

echo ""

# Build binary
print_info "Building CLI binary..."
cargo build --bin hash
print_status $? "CLI binary build"

echo ""

# Run tests
print_info "Running tests..."
cargo test --all-features
print_status $? "Tests"

echo ""

# Build documentation
print_info "Building documentation..."
cargo doc --no-deps
print_status $? "Documentation build"

echo ""

# Run examples
print_info "Testing examples..."

cargo run --example basic > /dev/null 2>&1
print_status $? "Example: basic"

cargo run --example file_integrity > /dev/null 2>&1
print_status $? "Example: file_integrity"

cargo run --example batch_processing > /dev/null 2>&1
print_status $? "Example: batch_processing"

echo ""

# Package check
print_info "Checking package..."
cargo package --allow-dirty --no-verify > /dev/null 2>&1
print_status $? "Package creation"

echo ""

# Build release
print_info "Building release binary..."
cargo build --release --bin hash
print_status $? "Release build"

echo ""

# Test CLI
print_info "Testing CLI functionality..."
./target/release/hash "test" -a sha256 > /dev/null 2>&1
print_status $? "CLI: basic hash"

./target/release/hash --list-algorithms > /dev/null 2>&1
print_status $? "CLI: list algorithms"

echo ""

# Security audit (if cargo-audit is installed)
if command -v cargo-audit &> /dev/null; then
    print_info "Running security audit..."
    cargo audit
    print_status $? "Security audit"
    echo ""
fi

# Check dependencies
print_info "Checking dependencies..."
cargo tree --depth 1

echo ""
echo "================================"
echo -e "${GREEN}All checks passed!${NC}"
echo "================================"
echo ""
echo "Ready for:"
echo "  - Local development"
echo "  - Publishing to crates.io"
echo "  - Production use"
echo ""
