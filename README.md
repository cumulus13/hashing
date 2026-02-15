# Hashing

[![Crates.io](https://img.shields.io/crates/v/hashing.svg)](https://crates.io/crates/hashing)
[![Documentation](https://docs.rs/hashing/badge.svg)](https://docs.rs/hashing)
[![License](https://img.shields.io/crates/l/hashing.svg)](https://github.com/cumulus13/hashing#license)

A robust, production-ready Rust library and CLI tool for generating cryptographic hashes. Supports multiple algorithms with zero-copy streaming for efficient processing of large files.

## Features

- **19 Hash Algorithms**: MD5, SHA-1, SHA-2 family, SHA-3 family, BLAKE2, BLAKE3, and Keccak variants
- **Flexible Input**: Hash strings or files with automatic detection
- **Streaming**: Memory-efficient processing of large files
- **Export**: Save results in text, JSON, or checksum formats
- **Verification**: Compare computed hashes against expected values
- **Production-Ready**: Comprehensive error handling, testing, and documentation

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
hashing = "0.1"
```

### As a CLI Tool

```bash
cargo install hashing
```

Or build from source:

```bash
git clone https://github.com/cumulus13/hashing
cd hashing
cargo install --path .
```

## CLI Usage

### Basic Usage

```bash
# Hash a string (SHA-256 by default)
hash "hello world"

# Hash a file
hash document.pdf

# Use a different algorithm
hash -a blake3 file.txt

# Force treat input as string (even if it matches a filename)
hash -s myfile.txt
```

### Available Algorithms

```bash
# List all supported algorithms
hash --list-algorithms

# Use specific algorithms
hash -a md5 file.txt
hash -a sha512 "my string"
hash -a blake3 large_file.bin
```

Supported algorithms:
- **MD5**: `md5` (legacy, insecure)
- **SHA-1**: `sha1` (legacy, insecure)
- **SHA-2**: `sha224`, `sha256`, `sha384`, `sha512`, `sha512-224`, `sha512-256`
- **SHA-3**: `sha3-224`, `sha3-256`, `sha3-384`, `sha3-512`
- **BLAKE2**: `blake2b`, `blake2s`
- **BLAKE3**: `blake3` (recommended for speed)
- **Keccak**: `keccak224`, `keccak256`, `keccak384`, `keccak512`

### Compute All Algorithms

```bash
# Hash with all algorithms
hash -A file.txt

# Quiet mode (only output hashes)
hash -A -q file.txt
```

### Export Results

```bash
# Export to text file
hash file.txt -e output.txt

# Export as JSON with metadata
hash file.txt -e result.json -f json

# Export in checksum format (compatible with sha256sum, md5sum, etc.)
hash file.txt -e file.sha256 -f checksum

# Export all algorithms
hash -A file.txt -e hashes
# Creates: hashes.md5, hashes.sha256, hashes.blake3, etc.
```

### Verify Hashes

```bash
# Verify file integrity
hash file.txt -c expected_hash_value

# Exit code 0 if match, 1 if mismatch
hash file.txt -c abc123... && echo "Valid" || echo "Invalid"
```

### Quiet Mode

```bash
# Only output the hash (useful for scripts)
hash -q file.txt

# Pipe to other commands
HASH=$(hash -q -a sha256 file.txt)
echo $HASH
```

## Library Usage

### Basic Hashing

```rust
use hashing::{hash_string, hash_file, Algorithm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hash a string
    let digest = hash_string("hello world", Algorithm::Sha256)?;
    println!("SHA-256: {}", digest);

    // Hash a file
    let file_hash = hash_file("document.pdf", Algorithm::Blake3)?;
    println!("BLAKE3: {}", file_hash);

    Ok(())
}
```

### Working with Hash Results

```rust
use hashing::{hash_string, Algorithm, HashResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digest = hash_string("test data", Algorithm::Sha256)?;
    
    let result = HashResult::new(
        Algorithm::Sha256,
        digest,
        "string"
    );

    // Export to JSON
    let json = result.to_json()?;
    println!("{}", json);

    // Export to text
    let text = result.to_text();
    println!("{}", text);

    Ok(())
}
```

### Hashing Multiple Algorithms

```rust
use hashing::{hash_bytes, Algorithm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = b"important data";

    for algorithm in Algorithm::all() {
        let digest = hash_bytes(data, algorithm)?;
        println!("{:<15} {}", algorithm.name(), digest);
    }

    Ok(())
}
```

### Custom Algorithm Selection

```rust
use hashing::{Algorithm, hash_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse algorithm from string
    let algo = Algorithm::from_str("sha3-256")?;
    let digest = hash_string("data", algo)?;
    
    println!("{}: {}", algo.name(), digest);
    Ok(())
}
```

## Performance

The library uses buffered I/O for efficient file processing:

- **Buffer size**: 8KB chunks for optimal I/O performance
- **Zero-copy**: Streaming processing without loading entire files into memory
- **BLAKE3**: Parallelized hashing for maximum throughput on modern CPUs

Benchmark (1GB file):
- BLAKE3: ~1.5 GB/s
- SHA-256: ~500 MB/s
- SHA-512: ~700 MB/s

## Security Considerations

⚠️ **MD5 and SHA-1 are cryptographically broken** and should only be used for legacy compatibility, not security.

**Recommended algorithms:**
- **General use**: SHA-256, SHA-512
- **Maximum speed**: BLAKE3
- **Cryptographic applications**: SHA-3 family
- **Compatibility**: SHA-256 (widely supported)

---

## Examples

See the [`examples/`](examples/) directory for more usage examples:

```bash
cargo run --example basic
cargo run --example file_integrity
cargo run --example batch_processing
```

```bash
$ hash -l

    Available hash algorithms:

      MD5             md5             128-bit (insecure, legacy use only)
      SHA-1           sha1            160-bit (insecure, legacy use only)
      SHA-224         sha224          224-bit SHA-2
      SHA-256         sha256          256-bit SHA-2 (recommended)
      SHA-384         sha384          384-bit SHA-2
      SHA-512         sha512          512-bit SHA-2
      SHA-512/224     sha512-224      224-bit SHA-2 variant
      SHA-512/256     sha512-256      256-bit SHA-2 variant
      SHA3-224        sha3-224        224-bit SHA-3
      SHA3-256        sha3-256        256-bit SHA-3
      SHA3-384        sha3-384        384-bit SHA-3
      SHA3-512        sha3-512        512-bit SHA-3
      BLAKE2b         blake2b         512-bit BLAKE2b
      BLAKE2s         blake2s         256-bit BLAKE2s
      BLAKE3          blake3          256-bit BLAKE3 (fast, modern)
      Keccak-224      keccak224       224-bit Keccak
      Keccak-256      keccak256       256-bit Keccak
      Keccak-384      keccak384       384-bit Keccak
      Keccak-512      keccak512       512-bit Keccak

$ hash -A TEST
    Computing hashes for all algorithms...

    MD5:            033bd94b1168d7e4f0d644c3c95e35bf
    SHA1:           984816fd329622876e14907634264e6f332e9fb3
    SHA224:         917ecca24f3e6ceaf52375d8083381f1f80a21e6e49fbadc40afeb8e
    SHA256:         94ee059335e587e501cc4bf90613e0814f00a7b08bc7c648fd865a2af6a22cc2
    SHA384:         4f37c49c0024445f91977dbc47bd4da9c4de8d173d03379ee19c2bb15435c2c7e624ea42f7cc1689961cb7aca50c7d17
    SHA512:         7bfa95a688924c47c7d22381f20cc926f524beacb13f84e203d4bd8cb6ba2fce81c57a5f059bf3d509926487bde925b3bcee0635e4f7baeba054e5dba696b2bf
    SHA512-224:     cafa3fa433ff2f0a13aa78163a8e871294bc576222bd3cb489505af0
    SHA512-256:     4a9534f239bfb947a1a647ef54ffb6cce8c04304983c97507dcac9229b4b836c
    SHA3-224:       01b1c8f4d4d96161641223362cf9874fc97ca778c112bffeabe757b7
    SHA3-256:       f4f08e9367e133dc42a4b9c9c665a9efbd4bf15db14d49c6ec51d0dc4c437ffb
    SHA3-384:       d035baa982c8bfea20a735ab02d7a2e12b0545ce1fbf75779f27a92cf3f757beb87c2141ba8a9d9c158d88985bedc213
    SHA3-512:       f9da82cb0e517cfc27498baacd053716e99adbaabc572f12a271d5d1348af11bdcab1b7836ea4ffbd88a88742532731055700966eab3dd5c01c3c80eddf5115b
    BLAKE2B:        5322bc39e200a6d2ef54ac6716376d5000f98a9715cb5293edd6e1e0f8865d3b22cb0fa92e09d52abef0cf58a2b067d4bc64fbee1e4bce0e9e642ce803dc6f99
    BLAKE2S:        e06fe76274d4e8b9f43d2bf0e095ee48eb6584afdc2baa98436081a4ea775cce
    BLAKE3:         3b2b74666c92cd76009bbee1acc3ae2652d3b4adeb1a5a85f949f0277e171f67
    KECCAK224:      0f14c1e2ff276c27d97b22cac8ffaf6ccb0cde61c4c0aa44ebd5cfc0
    KECCAK256:      852daa74cc3c31fe64542bb9b8764cfb91cc30f9acf9389071ffb44a9eefde46
    KECCAK384:      c0e33919602f27b8be4715aaae4b8d83e4d5ef8d6949c135d67ded867c1781dbe3805931ff7c51e9675ef317c487aac9
    KECCAK512:      7be7d328cb8af4416d78f626be5974d18787cb45248da750d8ba5401b11517f0ad43282294be90cb6a9369de1381ceee3f11488521a5d606abf28e05beb5bbaf

$ hash -a sha512-256 test.txt
Algorithm:  SHA512-256
Input type: file
File path:  test.txt
Hash:       0e433fb22780f375f0c00395aebdf7a7c19817775d0b4f0ece070631596b7a92

$ hash -a sha512-256 test.txt -e test.sha512
Algorithm:  SHA512-256
Input type: file
File path:  test.txt
Hash:       0e433fb22780f375f0c00395aebdf7a7c19817775d0b4f0ece070631596b7a92
Exported to: test.sha512

$ hash  -h
Generate cryptographic hashes for strings and files

Usage: hash.exe [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input: file path or string to hash

Options:
  -a, --algorithm <ALGORITHM>  Hash algorithm to use [default: sha256]
  -l, --list-algorithms        List all available algorithms
  -e, --export <FILE>          Export result to file
  -f, --format <FORMAT>        Export format [default: text] [possible values: text, json, checksum]
  -A, --all-algorithms         Compute hashes for all algorithms
  -s, --string                 Treat input as a string even if it matches a file path
  -q, --quiet                  Quiet mode - only output the hash
  -c, --verify <EXPECTED>      Verify hash against expected value
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

---


## Testing

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 👤 Author
        
[Hadi Cahyadi](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

## Acknowledgments

This project uses the following excellent crates:
- [RustCrypto](https://github.com/RustCrypto) - Cryptographic algorithm implementations
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) - Fast cryptographic hash function
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
