# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-02-15

### Added
- Initial release
- Support for 19 hash algorithms:
  - MD5
  - SHA-1
  - SHA-2 family (SHA-224, SHA-256, SHA-384, SHA-512, SHA-512/224, SHA-512/256)
  - SHA-3 family (SHA3-224, SHA3-256, SHA3-384, SHA3-512)
  - BLAKE2 (BLAKE2b-512, BLAKE2s-256)
  - BLAKE3
  - Keccak family (Keccak-224, Keccak-256, Keccak-384, Keccak-512)
- String and file hashing with automatic detection
- CLI tool (`hash`) with comprehensive options
- Export functionality (text, JSON, checksum formats)
- Hash verification against expected values
- Batch processing (all algorithms at once)
- Comprehensive error handling with `thiserror`
- Full documentation and examples
- Integration tests and benchmarks
- Zero-copy streaming for large files

### Features
- Memory-efficient file processing with 8KB buffers
- Production-ready error handling
- Cross-platform support
- Dual-licensed (MIT OR Apache-2.0)
