# Security and Best Practices

## Security Considerations

### Algorithm Selection

#### ⚠️ **Cryptographically Broken Algorithms**

**MD5** and **SHA-1** are cryptographically broken and should NOT be used for security purposes:

- **MD5**: Vulnerable to collision attacks since 2004. Only use for checksums in non-security contexts.
- **SHA-1**: Broken since 2017 (SHAttered attack). Avoid for any security application.

These algorithms are included only for:
- Legacy system compatibility
- Non-security checksums (e.g., cache keys)
- Historical data verification

#### ✅ **Recommended Algorithms**

**For general security use:**
- **SHA-256** or **SHA-512** (SHA-2 family) - Industry standard, widely supported
- **SHA3-256** or **SHA3-512** - Modern NIST standard
- **BLAKE3** - Fastest modern cryptographic hash, parallel processing

**For specific use cases:**
- **File integrity**: SHA-256, SHA-512, BLAKE3
- **Digital signatures**: SHA-256, SHA-512, SHA3-256
- **Password hashing**: ❌ Use Argon2, bcrypt, or scrypt instead (not provided by this library)
- **High-speed checksums**: BLAKE3
- **Compatibility**: SHA-256 (most widely supported)

### Timing Attack Resistance

This library does not implement constant-time operations. For applications requiring timing attack resistance (e.g., HMAC verification), use specialized constant-time comparison functions.

### Memory Safety

The library uses:
- Rust's memory safety guarantees
- Well-audited cryptographic implementations from RustCrypto
- Buffered I/O to prevent excessive memory usage

### Dependencies

All cryptographic primitives come from trusted sources:
- **RustCrypto**: Widely used, audited Rust cryptography implementations
- **BLAKE3**: Official implementation from the BLAKE3 team

Regular dependency updates are recommended to get security patches.

## Best Practices

### File Hashing

```rust
use hashing::{hash_file, Algorithm};

// ✅ Good: Use appropriate algorithm
let hash = hash_file("important.pdf", Algorithm::Sha256)?;

// ✅ Good: Use BLAKE3 for speed when security permits
let hash = hash_file("video.mp4", Algorithm::Blake3)?;

// ⚠️ Acceptable: MD5 only for checksums, not security
let hash = hash_file("cache.dat", Algorithm::Md5)?;

// ❌ Bad: Don't use MD5 or SHA-1 for security
// let hash = hash_file("contract.pdf", Algorithm::Md5)?; // NO!
```

### Export and Storage

```rust
use hashing::{hash_file, Algorithm, HashResult};

// Store hash with metadata for verification
let digest = hash_file("document.pdf", Algorithm::Sha256)?;
let result = HashResult::new(Algorithm::Sha256, digest, "file")
    .with_path("document.pdf");

// Export as JSON for structured storage
std::fs::write("document.hash", result.to_json()?)?;

// Or use checksum format for compatibility with sha256sum
let checksum = format!("{}  document.pdf", result.digest);
std::fs::write("document.sha256", checksum)?;
```

### Verification Workflow

```rust
use hashing::{hash_file, Algorithm};

fn verify_file_integrity(file: &str, expected: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let actual = hash_file(file, Algorithm::Sha256)?;
    
    // Case-insensitive comparison
    Ok(actual.eq_ignore_ascii_case(expected))
}

// Use it
match verify_file_integrity("download.zip", "abc123...") {
    Ok(true) => println!("✓ File verified"),
    Ok(false) => println!("✗ Verification failed - possible tampering!"),
    Err(e) => println!("Error: {}", e),
}
```

### Batch Processing

```rust
use hashing::{hash_file, Algorithm};
use std::path::Path;

fn hash_directory(dir: &Path, algo: Algorithm) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let hash = hash_file(&path, algo)?;
            results.push((
                path.display().to_string(),
                hash
            ));
        }
    }
    
    Ok(results)
}
```

### Performance Optimization

```rust
use hashing::{Algorithm, hash_file};

// For large files, BLAKE3 is significantly faster
let start = std::time::Instant::now();
let hash = hash_file("large_video.mp4", Algorithm::Blake3)?;
println!("BLAKE3: {:?}", start.elapsed());

// SHA-256 is slower but more widely supported
let start = std::time::Instant::now();
let hash = hash_file("large_video.mp4", Algorithm::Sha256)?;
println!("SHA-256: {:?}", start.elapsed());

// Use BLAKE3 when:
// - Processing large files (> 10 MB)
// - Speed is critical
// - Modern systems with multiple cores
//
// Use SHA-256 when:
// - Compatibility is required
// - Integration with existing tools
// - Standard compliance needed
```

### Error Handling

```rust
use hashing::{hash_file, Algorithm, HashError};

fn robust_hash_file(path: &str) -> Result<String, String> {
    hash_file(path, Algorithm::Sha256)
        .map_err(|e| match e {
            HashError::Io(io_err) => format!("File error: {}", io_err),
            HashError::UnsupportedAlgorithm(algo) => format!("Unsupported: {}", algo),
            HashError::InvalidInput(msg) => format!("Invalid input: {}", msg),
            HashError::ExportError(msg) => format!("Export failed: {}", msg),
        })
}
```

## CLI Best Practices

### Verification Scripts

```bash
#!/bin/bash
# Verify file integrity

FILE="important_document.pdf"
EXPECTED_HASH="abc123def456..."

if hash "$FILE" -c "$EXPECTED_HASH" -q; then
    echo "✓ File verified successfully"
    exit 0
else
    echo "✗ File verification FAILED"
    exit 1
fi
```

### Batch Verification

```bash
# Generate checksums for all files
find . -type f -exec hash {} -a sha256 -e {}.sha256 \;

# Verify all checksums
for file in *.sha256; do
    original="${file%.sha256}"
    expected=$(cat "$file")
    if hash "$original" -c "$expected" -q; then
        echo "✓ $original"
    else
        echo "✗ $original FAILED"
    fi
done
```

### Pipeline Integration

```bash
# Use in scripts
FILE_HASH=$(hash myfile.txt -q -a sha256)
echo "Hash: $FILE_HASH"

# Upload verification
UPLOADED_HASH=$(hash upload.zip -q)
EXPECTED_HASH=$(cat upload.zip.sha256)

if [ "$UPLOADED_HASH" = "$EXPECTED_HASH" ]; then
    echo "Upload verified"
else
    echo "Upload corrupted"
    exit 1
fi
```

## Production Deployment

### Recommendations

1. **Always verify downloads**
   ```bash
   hash downloaded_file.tar.gz -c "$EXPECTED_HASH"
   ```

2. **Store hashes securely**
   - Keep hash manifests in version control
   - Sign critical hash files
   - Use separate channels for hash distribution

3. **Regular verification**
   - Schedule periodic integrity checks
   - Verify backups before/after transfer
   - Check file integrity after network transfer

4. **Logging**
   ```rust
   let hash = hash_file(&path, Algorithm::Sha256)?;
   log::info!("Computed hash for {}: {}", path, hash);
   ```

5. **Monitoring**
   - Track hash computation times
   - Monitor for unexpected hash changes
   - Alert on verification failures

## Common Pitfalls

### ❌ Don't

```rust
// Don't use MD5 for security
let hash = hash_file("password_db.bin", Algorithm::Md5)?; // INSECURE!

// Don't ignore errors
let hash = hash_file("important.pdf", Algorithm::Sha256).unwrap(); // FRAGILE!

// Don't use for passwords
let hash = hash_string("password123", Algorithm::Sha256)?; // WRONG USE CASE!
```

### ✅ Do

```rust
// Use SHA-256 or better for security
let hash = hash_file("password_db.bin", Algorithm::Sha256)?;

// Handle errors properly
let hash = hash_file("important.pdf", Algorithm::Sha256)
    .context("Failed to hash PDF")?;

// Use proper password hashing (not this library)
// Use Argon2, bcrypt, or scrypt for passwords
```

## Compliance and Standards

- **FIPS 140-2**: SHA-256, SHA-512, SHA-3 variants are FIPS approved
- **NIST**: Recommends SHA-256 or stronger for digital signatures
- **Common Criteria**: SHA-2 and SHA-3 families meet requirements

## Support and Updates

- **Security issues**: Report to cumulus13@gmail.com
- **Updates**: Check for dependency updates regularly
- **Audits**: Run `cargo audit` periodically

## References

- [NIST Hash Functions](https://csrc.nist.gov/projects/hash-functions)
- [RustCrypto Hashes](https://github.com/RustCrypto/hashes)
- [BLAKE3 Specification](https://github.com/BLAKE3-team/BLAKE3-specs)
