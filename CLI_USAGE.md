# CLI Usage Guide

## Quick Reference

```bash
hash [OPTIONS] <INPUT>
```

## Common Usage Examples

### Basic Hashing

```bash
# Hash a string (default: SHA-256)
hash "hello world"

# Hash a file
hash document.pdf

# Use a different algorithm
hash -a blake3 file.txt
hash --algorithm md5 "my string"
```

### Hash with ALL Algorithms (−A flag)

```bash
# Compute ALL 19 hash algorithms at once
hash -A myfile.txt

# Output:
# Computing hashes for all algorithms...
#
# MD5:            5d41402abc4b2a76b9719d911017c592
# SHA1:           aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d
# SHA224:         ea09ae9cc6768c50fcee903ed054556e5bfc8347907f12598aa24193
# SHA256:         2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
# ... (all 19 algorithms)

# Quiet mode - just the hashes
hash -A -q myfile.txt

# Export all hashes to files
hash -A myfile.txt -e hashes
# Creates: hashes.md5, hashes.sha256, hashes.blake3, etc.

# Export all to JSON
hash -A myfile.txt -e results.json -f json
```

### Export Results

```bash
# Export to text file
hash file.txt -e hash.txt

# Export as JSON with metadata
hash file.txt -e result.json -f json

# Export in checksum format (compatible with sha256sum)
hash file.txt -e file.sha256 -f checksum
```

### Verify Hashes

```bash
# Verify file integrity
hash file.zip -c abc123def456...

# Exit code 0 if match, 1 if mismatch
hash file.zip -c $(cat expected.sha256) && echo "Valid"

# Quiet mode for scripting
hash file.zip -c abc123... -q
echo $?  # 0 = match, 1 = mismatch
```

### Compare Two Files or Strings

```bash
# Compare two files by hash (default: SHA-256)
hash file1.txt -C file2.txt

# Output:
# Comparing using SHA256
#
# Input 1: file1.txt (file)
# Hash 1:  2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
#
# Input 2: file2.txt (file)
# Hash 2:  2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
#
# ✓ MATCH - Inputs are identical

# Compare using specific algorithm
hash file1.zip -C file2.zip -a blake3

# Compare with ALL algorithms (comprehensive verification)
hash file1.txt -C file2.txt -A

# Output:
# Comparing with all algorithms...
#
# ✓ MD5:            MATCH
# ✓ SHA1:           MATCH
# ✓ SHA256:         MATCH
# ✓ SHA512:         MATCH
# ✓ BLAKE3:         MATCH
# ... (all 19 algorithms)
#
# Results: 19 matches, 0 mismatches
# ✓ ALL ALGORITHMS MATCH - Inputs are identical

# Quiet mode for scripting
hash backup.tar -C backup-copy.tar -C -q
echo $?  # 0 = identical, 1 = different

# Compare strings
hash "hello" -C "world" -s
# ✗ NO MATCH - Inputs are different

# Compare file vs string
hash config.json -C "some text" -s
```

### List Available Algorithms

```bash
# Show all 19 supported algorithms
hash --list-algorithms
hash -l

# Output:
# Available hash algorithms:
#
#   MD5             md5             128-bit (insecure, legacy use only)
#   SHA-1           sha1            160-bit (insecure, legacy use only)
#   SHA-256         sha256          256-bit SHA-2 (recommended)
#   BLAKE3          blake3          256-bit BLAKE3 (fast, modern)
#   ... (all 19 algorithms)
```

## All Options

| Option | Short | Description | Example |
|--------|-------|-------------|---------|
| `<INPUT>` | - | File path or string to hash | `hash file.txt` |
| `--algorithm` | `-a` | Algorithm to use | `-a blake3` |
| `--all-algorithms` | `-A` | Compute ALL algorithms | `-A` |
| `--compare` | `-C` | Compare two inputs by hash | `-C file2.txt` |
| `--string` | `-s` | Force treat as string | `-s myfile.txt` |
| `--export` | `-e` | Export to file | `-e output.txt` |
| `--format` | `-f` | Export format (text/json/checksum) | `-f json` |
| `--verify` | `-c` | Verify against expected hash | `-c abc123...` |
| `--quiet` | `-q` | Quiet mode (hash only) | `-q` |
| `--list-algorithms` | `-l` | List all algorithms | `-l` |
| `--help` | `-h` | Show help | `-h` |
| `--version` | `-V` | Show version | `-V` |

## Supported Algorithms (19 total)

### Recommended for Security
- **SHA-256** (default) - Industry standard
- **SHA-512** - Higher security
- **SHA3-256** - Modern NIST standard
- **BLAKE3** - Fastest modern hash

### Fast & Modern
- **BLAKE3** - Parallelized, very fast
- **BLAKE2b** - Fast, cryptographically secure
- **BLAKE2s** - Optimized for 8-32 bit platforms

### SHA-2 Family
- SHA-224, SHA-256, SHA-384, SHA-512
- SHA-512/224, SHA-512/256

### SHA-3 Family
- SHA3-224, SHA3-256, SHA3-384, SHA3-512

### Keccak (Ethereum)
- Keccak-224, Keccak-256, Keccak-384, Keccak-512

### Legacy (Insecure)
- **MD5** - Only for checksums, NOT security
- **SHA-1** - Only for checksums, NOT security

## Advanced Examples

### Batch Processing with -A

```bash
# Hash all files in directory with all algorithms
for file in *.pdf; do
    echo "Processing $file..."
    hash -A "$file" -e "hashes/${file}.json" -f json
done

# Create checksums for distribution
hash -A release.zip -e release
# Creates: release.md5, release.sha256, release.sha512, etc.
```

### Pipeline Integration

```bash
# Get just the hash for scripting
HASH=$(hash -q file.txt)
echo "File hash: $HASH"

# Verify downloaded file
EXPECTED=$(curl -s https://example.com/file.sha256)
hash downloaded.zip -c "$EXPECTED" || {
    echo "Checksum mismatch! Possible corruption or tampering."
    exit 1
}

# Compare two files
hash file1.txt -C file2.txt -q || echo "Files are different"

# Quick identity check
if hash original.zip -C backup.zip -q; then
    echo "✓ Backup verified - files are identical"
else
    echo "✗ Backup differs from original!"
    exit 1
fi
```

### File Comparison Workflows

```bash
# Verify backup integrity
hash /data/important.db -C /backup/important.db -A

# Compare downloaded file with original
hash original.iso -C downloaded.iso -a blake3

# Batch comparison
for file in originals/*; do
    backup="backups/$(basename $file)"
    if hash "$file" -C "$backup" -q; then
        echo "✓ $file matches backup"
    else
        echo "✗ $file differs from backup"
    fi
done

# Deduplication - find identical files
for file1 in dir1/*; do
    for file2 in dir2/*; do
        if hash "$file1" -C "$file2" -q; then
            echo "Duplicate found: $file1 == $file2"
        fi
    done
done
```

### Security Best Practices

```bash
# GOOD: Use SHA-256 or better for security
hash -a sha256 important.pdf
hash -a blake3 large_file.zip  # Fastest

# BAD: Don't use MD5/SHA-1 for security
hash -a md5 secret.txt  # ❌ Insecure!

# GOOD: Verify with multiple algorithms
hash -A sensitive.exe -e verification.json -f json
```

### Force String Mode

```bash
# If you have a file named "test.txt" but want to hash the string "test.txt"
hash -s "test.txt"

# Hash a filename as a string, not the file content
hash -s "config.json"
```

## Output Formats

### Text Format (default)
```
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

### JSON Format
```json
{
  "algorithm": "sha256",
  "digest": "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
  "input_type": "file",
  "input_path": "document.pdf"
}
```

### Checksum Format (compatible with sha256sum, md5sum, etc.)
```
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824  document.pdf
```

## Exit Codes

- **0** - Success (or verification passed)
- **1** - Error or verification failed

```bash
# Use in scripts
if hash file.txt -c expected_hash -q; then
    echo "Verification passed"
else
    echo "Verification FAILED"
fi
```

## Performance Tips

1. **Use BLAKE3 for speed**: `hash -a blake3 large_file.zip`
2. **Use -q for scripting**: Reduces output overhead
3. **Single algorithm is faster**: Only use `-A` when you need all hashes
4. **Stream large files**: The tool automatically streams files efficiently

## Common Workflows

### Download Verification
```bash
# Download file and checksum
wget https://example.com/file.zip
wget https://example.com/file.zip.sha256

# Verify
hash file.zip -c $(cat file.zip.sha256) && echo "✓ Verified"
```

### Release Signing
```bash
# Create multiple checksums for release
hash -A myapp-v1.0.tar.gz -e checksums/myapp-v1.0

# Users can verify with any algorithm
hash myapp-v1.0.tar.gz -c $(cat checksums/myapp-v1.0.sha256)
```

### File Deduplication
```bash
# Find duplicate files
for file in *; do
    echo "$(hash -q $file)  $file"
done | sort | uniq -w 64 -D
```

## Tips

💡 **Tip**: Use `-A` (all algorithms) for important files to provide maximum verification flexibility

💡 **Tip**: BLAKE3 is ~3x faster than SHA-256 for large files

💡 **Tip**: Export to JSON format (`-f json`) for structured data processing

💡 **Tip**: Use checksum format (`-f checksum`) for compatibility with standard Unix tools

⚠️ **Warning**: MD5 and SHA-1 are cryptographically broken - use only for non-security purposes

## Getting Help

```bash
# Show help
hash --help
hash -h

# Show version
hash --version
hash -V

# List all algorithms
hash --list-algorithms
hash -l
```

## Examples by Use Case

### Developer: Verify Build Artifacts
```bash
hash -A myapp.exe -e release/checksums.json -f json
```

### Security: Verify Downloaded Software
```bash
hash downloaded.dmg -c $(cat official.sha256) || exit 1
```

### Backup: Verify File Integrity
```bash
# Create checksums
hash -A backup.tar.gz -e backup.hashes

# Later: compare backup with original
hash original.tar.gz -C backup.tar.gz -A
```

### DevOps: CI/CD Pipeline
```bash
#!/bin/bash
HASH=$(hash -q -a blake3 release.zip)
echo "RELEASE_HASH=$HASH" >> $GITHUB_ENV
```

### System Admin: File Monitoring
```bash
#!/bin/bash
# Check if critical system file has changed
if ! hash /etc/passwd -C /backup/passwd.bak -q; then
    echo "ALERT: /etc/passwd has been modified!"
    mail -s "Security Alert" admin@example.com
fi
```

### Quality Assurance: Media File Verification
```bash
# Verify video rendering output matches
hash original.mp4 -C rendered.mp4 -a blake3 || {
    echo "Render output differs from source!"
    exit 1
}
```

---

**For more information, see the README.md or visit https://github.com/cumulus13/hashing**
