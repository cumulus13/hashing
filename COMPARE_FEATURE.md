# Compare Feature - Quick Reference

## What's New

Added **`-C` / `--compare`** flag to compare two files or strings by their hashes!

## Usage

### Basic Comparison

```bash
# Compare two files (default: SHA-256)
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
```

### Compare with Specific Algorithm

```bash
hash original.zip -C backup.zip -a blake3
hash file1.pdf -C file2.pdf -a sha512
```

### Compare with ALL Algorithms

```bash
hash file1.txt -C file2.txt -A

# Output:
# Comparing with all algorithms...
#
# ✓ MD5:            MATCH
# ✓ SHA1:           MATCH  
# ✓ SHA256:         MATCH
# ✓ SHA512:         MATCH
# ✓ BLAKE3:         MATCH
# ✓ SHA3-256:       MATCH
# ... (all 19 algorithms)
#
# Results: 19 matches, 0 mismatches
# ✓ ALL ALGORITHMS MATCH - Inputs are identical
```

### Quiet Mode for Scripting

```bash
# Exit code 0 if identical, 1 if different
hash file1.db -C file2.db -q
if [ $? -eq 0 ]; then
    echo "Files are identical"
else
    echo "Files differ"
fi

# One-liner
hash backup.tar -C backup-copy.tar -q && echo "✓ Verified" || echo "✗ Different"
```

### Compare Strings

```bash
# Compare string literals
hash "hello" -C "world" -s

# Compare file content as strings
hash config1.json -C config2.json -s
```

## Use Cases

### 1. Backup Verification
```bash
# Verify backup matches original
hash /data/critical.db -C /backup/critical.db -A

# Batch verify all backups
for file in /data/*; do
    backup="/backup/$(basename $file)"
    if hash "$file" -C "$backup" -q; then
        echo "✓ $(basename $file)"
    else  
        echo "✗ $(basename $file) - MISMATCH!"
    fi
done
```

### 2. Deduplication
```bash
# Find duplicate files
for file1 in dir1/*; do
    for file2 in dir2/*; do
        if hash "$file1" -C "$file2" -q; then
            echo "Duplicate: $file1 == $file2"
        fi
    done
done
```

### 3. File Monitoring
```bash
#!/bin/bash
# Alert if critical file changes
SENTINEL="/etc/critical-config"
BASELINE="/baseline/critical-config"

if ! hash "$SENTINEL" -C "$BASELINE" -q; then
    echo "ALERT: $SENTINEL has been modified!"
    # Send notification, log, etc.
fi
```

### 4. Build Verification
```bash
# Verify build is reproducible
hash build/app-v1.0.exe -C build-ci/app-v1.0.exe -a sha256 || {
    echo "Build not reproducible!"
    exit 1
}
```

### 5. Download Verification
```bash
# Compare downloaded file with cached version
if hash downloads/package.tar.gz -C cache/package.tar.gz -q; then
    echo "Using cached version"
    cp cache/package.tar.gz ./
else
    echo "New version downloaded"
    cp downloads/package.tar.gz cache/
fi
```

## Exit Codes

- **0** - Files/strings are identical
- **1** - Files/strings are different

```bash
hash file1.txt -C file2.txt -q
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    echo "Identical"
else
    echo "Different"  
fi
```

## Advantages Over Manual Comparison

### Before (Manual):
```bash
HASH1=$(hash -q file1.txt)
HASH2=$(hash -q file2.txt)
if [ "$HASH1" = "$HASH2" ]; then
    echo "Match"
fi
```

### After (Built-in):
```bash
hash file1.txt -C file2.txt -q && echo "Match"
```

**Benefits:**
- ✅ Cleaner syntax
- ✅ Better output formatting
- ✅ Automatic file/string detection
- ✅ Works with `-A` for comprehensive verification
- ✅ Proper exit codes

## Combining with Other Flags

```bash
# Compare + All algorithms + Export
hash file1.zip -C file2.zip -A -e comparison.json -f json

# Compare + Specific algorithm + Quiet
hash backup.db -C original.db -a blake3 -q

# Compare + String mode
hash "text1" -C "text2" -s
```

## Tips

💡 **Use `-A`** (all algorithms) for critical verification to ensure comprehensive matching

💡 **Use quiet mode** (`-q`) in scripts for clean exit code checking  

💡 **Use BLAKE3** (`-a blake3`) for fastest comparisons of large files

💡 **Exit codes** make it perfect for CI/CD pipelines and automation

## Complete Syntax

```
hash <INPUT1> --compare <INPUT2> [OPTIONS]
hash <INPUT1> -C <INPUT2> [OPTIONS]

Options:
  -a, --algorithm <ALGO>   Algorithm to use (default: sha256)
  -A, --all-algorithms     Compare with all 19 algorithms
  -s, --string            Treat both inputs as strings
  -q, --quiet             Quiet mode (exit code only)
  -e, --export <FILE>     Export comparison results
  -f, --format <FORMAT>   Export format (text/json/checksum)
```

## See Also

- `CLI_USAGE.md` - Complete CLI documentation
- `README.md` - Full project documentation
- `hash --help` - Built-in help

---

**New in version 0.1.0** - Compare feature added!
