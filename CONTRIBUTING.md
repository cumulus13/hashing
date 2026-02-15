# Contributing to Hashing

Thank you for your interest in contributing to the Hashing project! We welcome contributions from everyone.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/hashing
   cd hashing
   ```
3. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Building
```bash
cargo build
```

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Running Benchmarks
```bash
cargo bench
```

### Running Examples
```bash
cargo run --example basic
cargo run --example file_integrity
cargo run --example batch_processing
```

## Code Style

- Follow the official Rust style guide
- Use `rustfmt` to format code:
  ```bash
  cargo fmt
  ```
- Run `clippy` for linting:
  ```bash
  cargo clippy -- -D warnings
  ```

## Adding New Hash Algorithms

To add a new hash algorithm:

1. Add the dependency to `Cargo.toml`
2. Add the algorithm variant to the `Algorithm` enum in `src/lib.rs`
3. Implement the algorithm in `hash_bytes()` and `hash_file()` functions
4. Add the algorithm name to `Algorithm::name()` and `Algorithm::from_str()`
5. Add tests for the new algorithm
6. Update documentation

## Testing Guidelines

- Write tests for all new features
- Ensure all tests pass before submitting PR
- Add integration tests for new functionality
- Include edge cases in tests
- Test with various input sizes

## Documentation

- Add doc comments for all public APIs
- Include examples in doc comments
- Update README.md if needed
- Add entries to CHANGELOG.md

## Pull Request Process

1. Update documentation and tests
2. Ensure all tests pass
3. Run `cargo fmt` and `cargo clippy`
4. Update CHANGELOG.md with your changes
5. Submit pull request with clear description
6. Wait for review and address feedback

## Code Review

All submissions require review. We use GitHub pull requests for this purpose.

## Reporting Bugs

Please use GitHub Issues to report bugs. Include:
- Description of the bug
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information (OS, Rust version)
- Relevant logs or error messages

## Feature Requests

We welcome feature requests! Please use GitHub Issues and include:
- Clear description of the feature
- Use cases and motivation
- Example usage (if applicable)

## Questions?

Feel free to open an issue for questions or reach out to the maintainer at cumulus13@gmail.com

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
