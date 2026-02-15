use hashing::{hash_file, hash_string, Algorithm, HashResult};
use std::io::Write;
use std::str::FromStr;
use tempfile::NamedTempFile;

#[test]
fn test_all_algorithms_string() {
    let input = "test data for hashing";
    
    for algorithm in Algorithm::all() {
        let result = hash_string(input, algorithm);
        assert!(
            result.is_ok(),
            "Algorithm {:?} failed to hash string",
            algorithm
        );
        
        let digest = result.unwrap();
        assert!(!digest.is_empty(), "Empty digest for {:?}", algorithm);
        assert!(
            digest.chars().all(|c| c.is_ascii_hexdigit()),
            "Non-hex characters in digest for {:?}",
            algorithm
        );
    }
}

#[test]
fn test_sha256_known_values() {
    let test_cases = vec![
        ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        ("hello", "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
        ("hello world", "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"),
        ("The quick brown fox jumps over the lazy dog", 
         "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"),
    ];

    for (input, expected) in test_cases {
        let result = hash_string(input, Algorithm::Sha256).unwrap();
        assert_eq!(result, expected, "SHA-256 mismatch for input: {}", input);
    }
}

#[test]
fn test_md5_known_values() {
    let test_cases = vec![
        ("", "d41d8cd98f00b204e9800998ecf8427e"),
        ("hello", "5d41402abc4b2a76b9719d911017c592"),
        ("hello world", "5eb63bbbe01eeed093cb22bb8f5acdc3"),
    ];

    for (input, expected) in test_cases {
        let result = hash_string(input, Algorithm::Md5).unwrap();
        assert_eq!(result, expected, "MD5 mismatch for input: {}", input);
    }
}

#[test]
fn test_blake3_known_values() {
    let result = hash_string("hello world", Algorithm::Blake3).unwrap();
    // BLAKE3 should produce 64 hex characters (32 bytes)
    assert_eq!(result.len(), 64);
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_file_hashing() -> Result<(), Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    let test_data = b"This is test file content for hashing";
    temp_file.write_all(test_data)?;
    temp_file.flush()?;

    // Hash the file
    let file_hash = hash_file(temp_file.path(), Algorithm::Sha256)?;
    
    // Hash the same data as a string
    let string_hash = hash_string(
        std::str::from_utf8(test_data)?,
        Algorithm::Sha256
    )?;

    assert_eq!(file_hash, string_hash, "File hash should match string hash");
    Ok(())
}

#[test]
fn test_large_file_hashing() -> Result<(), Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    
    // Create a 1MB file
    let chunk = vec![0xAB; 1024];
    for _ in 0..1024 {
        temp_file.write_all(&chunk)?;
    }
    temp_file.flush()?;

    // Should handle large files efficiently
    let result = hash_file(temp_file.path(), Algorithm::Sha256);
    assert!(result.is_ok());
    
    let digest = result.unwrap();
    assert_eq!(digest.len(), 64); // SHA-256 produces 64 hex chars
    
    Ok(())
}

#[test]
fn test_algorithm_parsing() {
    let test_cases = vec![
        ("sha256", Algorithm::Sha256),
        ("SHA256", Algorithm::Sha256),
        ("md5", Algorithm::Md5),
        ("blake3", Algorithm::Blake3),
        ("sha512-256", Algorithm::Sha512_256),
        ("sha512_256", Algorithm::Sha512_256),
        ("sha3-256", Algorithm::Sha3_256),
        ("sha3_256", Algorithm::Sha3_256),
    ];

    for (input, expected) in test_cases {
        let result = Algorithm::from_str(input);
        assert!(result.is_ok(), "Failed to parse: {}", input);
        assert_eq!(result.unwrap(), expected, "Mismatch for: {}", input);
    }

    // Test invalid algorithm
    assert!(Algorithm::from_str("invalid_algo").is_err());
}

#[test]
fn test_hash_result_json() -> Result<(), Box<dyn std::error::Error>> {
    let result = HashResult::new(
        Algorithm::Sha256,
        "abcd1234".to_string(),
        "string"
    );

    let json = result.to_json()?;
    assert!(json.contains("sha256"));
    assert!(json.contains("abcd1234"));
    assert!(json.contains("string"));

    // Should be valid JSON
    let _: HashResult = serde_json::from_str(&json)?;
    Ok(())
}

#[test]
fn test_hash_result_with_path() {
    let result = HashResult::new(
        Algorithm::Sha256,
        "hash123".to_string(),
        "file"
    ).with_path("/path/to/file.txt");

    assert_eq!(result.input_path, Some("/path/to/file.txt".to_string()));
}

#[test]
fn test_empty_string() {
    for algorithm in Algorithm::all() {
        let result = hash_string("", algorithm);
        assert!(result.is_ok(), "Failed to hash empty string with {:?}", algorithm);
    }
}

#[test]
fn test_unicode_string() {
    let unicode_strings = vec![
        "Hello, 世界",
        "Привет мир",
        "مرحبا بالعالم",
        "🚀🔒💻",
    ];

    for input in unicode_strings {
        for algorithm in &[Algorithm::Sha256, Algorithm::Blake3] {
            let result = hash_string(input, *algorithm);
            assert!(result.is_ok(), "Failed to hash unicode: {}", input);
        }
    }
}

#[test]
fn test_consistency() {
    let input = "consistency test";
    
    for algorithm in Algorithm::all() {
        let hash1 = hash_string(input, algorithm).unwrap();
        let hash2 = hash_string(input, algorithm).unwrap();
        assert_eq!(hash1, hash2, "Inconsistent hashing for {:?}", algorithm);
    }
}

#[test]
fn test_different_inputs_different_hashes() {
    let inputs = vec!["input1", "input2", "input3"];
    
    for algorithm in &[Algorithm::Sha256, Algorithm::Blake3] {
        let mut hashes = std::collections::HashSet::new();
        
        for input in &inputs {
            let hash = hash_string(input, *algorithm).unwrap();
            assert!(hashes.insert(hash), "Collision detected for {:?}", algorithm);
        }
    }
}

#[test]
fn test_sha3_family() {
    let input = "SHA-3 test";
    
    let sha3_algorithms = vec![
        Algorithm::Sha3_224,
        Algorithm::Sha3_256,
        Algorithm::Sha3_384,
        Algorithm::Sha3_512,
    ];

    for algorithm in sha3_algorithms {
        let result = hash_string(input, algorithm);
        assert!(result.is_ok(), "SHA-3 algorithm {:?} failed", algorithm);
    }
}

#[test]
fn test_blake2_family() {
    let input = "BLAKE2 test";
    
    let blake2_algorithms = vec![
        Algorithm::Blake2b512,
        Algorithm::Blake2s256,
    ];

    for algorithm in blake2_algorithms {
        let result = hash_string(input, algorithm);
        assert!(result.is_ok(), "BLAKE2 algorithm {:?} failed", algorithm);
    }
}

#[test]
fn test_keccak_family() {
    let input = "Keccak test";
    
    let keccak_algorithms = vec![
        Algorithm::Keccak224,
        Algorithm::Keccak256,
        Algorithm::Keccak384,
        Algorithm::Keccak512,
    ];

    for algorithm in keccak_algorithms {
        let result = hash_string(input, algorithm);
        assert!(result.is_ok(), "Keccak algorithm {:?} failed", algorithm);
    }
}
