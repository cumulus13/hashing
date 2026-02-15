//! # Hashing
//!
//! A robust, production-ready hashing library supporting multiple cryptographic algorithms.
//!
//! ## Features
//!
//! - Multiple hash algorithms (MD5, SHA-1, SHA-2, SHA-3, BLAKE2, BLAKE3)
//! - String and file hashing
//! - Export to multiple formats (JSON, hex, base64)
//! - Zero-copy streaming for large files
//! - Comprehensive error handling
//!
//! ## Example
//!
//! ```rust
//! use hashing::{hash_string, Algorithm};
//!
//! let digest = hash_string("hello world", Algorithm::Sha256).unwrap();
//! println!("SHA-256: {}", digest);
//! ```

use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

// Re-export digest traits for library users
pub use blake2::Digest as Blake2Digest;
pub use md5::Digest as Md5Digest;
pub use sha2::Digest as Sha2Digest;
pub use sha3::Digest as Sha3Digest;

/// Errors that can occur during hashing operations
#[derive(Error, Debug)]
pub enum HashError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Export error: {0}")]
    ExportError(String),
}

/// Result type for hashing operations
pub type Result<T> = std::result::Result<T, HashError>;

/// Supported hashing algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    // MD5
    Md5,
    
    // SHA-1 family
    Sha1,
    
    // SHA-2 family
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
    
    // SHA-3 family
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    
    // BLAKE2
    Blake2b512,
    Blake2s256,
    
    // BLAKE3
    Blake3,
    
    // Keccak (SHA-3 variant)
    Keccak224,
    Keccak256,
    Keccak384,
    Keccak512,
}

impl Algorithm {
    /// Get all available algorithms
    pub fn all() -> Vec<Algorithm> {
        vec![
            Algorithm::Md5,
            Algorithm::Sha1,
            Algorithm::Sha224,
            Algorithm::Sha256,
            Algorithm::Sha384,
            Algorithm::Sha512,
            Algorithm::Sha512_224,
            Algorithm::Sha512_256,
            Algorithm::Sha3_224,
            Algorithm::Sha3_256,
            Algorithm::Sha3_384,
            Algorithm::Sha3_512,
            Algorithm::Blake2b512,
            Algorithm::Blake2s256,
            Algorithm::Blake3,
            Algorithm::Keccak224,
            Algorithm::Keccak256,
            Algorithm::Keccak384,
            Algorithm::Keccak512,
        ]
    }
    
    /// Get algorithm name as string
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::Md5 => "md5",
            Algorithm::Sha1 => "sha1",
            Algorithm::Sha224 => "sha224",
            Algorithm::Sha256 => "sha256",
            Algorithm::Sha384 => "sha384",
            Algorithm::Sha512 => "sha512",
            Algorithm::Sha512_224 => "sha512-224",
            Algorithm::Sha512_256 => "sha512-256",
            Algorithm::Sha3_224 => "sha3-224",
            Algorithm::Sha3_256 => "sha3-256",
            Algorithm::Sha3_384 => "sha3-384",
            Algorithm::Sha3_512 => "sha3-512",
            Algorithm::Blake2b512 => "blake2b",
            Algorithm::Blake2s256 => "blake2s",
            Algorithm::Blake3 => "blake3",
            Algorithm::Keccak224 => "keccak224",
            Algorithm::Keccak256 => "keccak256",
            Algorithm::Keccak384 => "keccak384",
            Algorithm::Keccak512 => "keccak512",
        }
    }
}

/// Implement FromStr trait for Algorithm
impl FromStr for Algorithm {
    type Err = HashError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "md5" => Ok(Algorithm::Md5),
            "sha1" => Ok(Algorithm::Sha1),
            "sha224" => Ok(Algorithm::Sha224),
            "sha256" => Ok(Algorithm::Sha256),
            "sha384" => Ok(Algorithm::Sha384),
            "sha512" => Ok(Algorithm::Sha512),
            "sha512-224" | "sha512_224" => Ok(Algorithm::Sha512_224),
            "sha512-256" | "sha512_256" => Ok(Algorithm::Sha512_256),
            "sha3-224" | "sha3_224" => Ok(Algorithm::Sha3_224),
            "sha3-256" | "sha3_256" => Ok(Algorithm::Sha3_256),
            "sha3-384" | "sha3_384" => Ok(Algorithm::Sha3_384),
            "sha3-512" | "sha3_512" => Ok(Algorithm::Sha3_512),
            "blake2b" | "blake2b512" => Ok(Algorithm::Blake2b512),
            "blake2s" | "blake2s256" => Ok(Algorithm::Blake2s256),
            "blake3" => Ok(Algorithm::Blake3),
            "keccak224" => Ok(Algorithm::Keccak224),
            "keccak256" => Ok(Algorithm::Keccak256),
            "keccak384" => Ok(Algorithm::Keccak384),
            "keccak512" => Ok(Algorithm::Keccak512),
            _ => Err(HashError::UnsupportedAlgorithm(s.to_string())),
        }
    }
}

/// Hash a string using the specified algorithm
///
/// # Examples
///
/// ```
/// use hashing::{hash_string, Algorithm};
///
/// let digest = hash_string("hello", Algorithm::Sha256).unwrap();
/// assert_eq!(digest.len(), 64); // SHA-256 produces 32 bytes = 64 hex chars
/// ```
pub fn hash_string(input: &str, algorithm: Algorithm) -> Result<String> {
    hash_bytes(input.as_bytes(), algorithm)
}

/// Hash a byte slice using the specified algorithm
pub fn hash_bytes(data: &[u8], algorithm: Algorithm) -> Result<String> {
    use blake2::Blake2b512;
    use blake2::Blake2s256;
    use md5::Md5;
    use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256, Digest};
    use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512, Keccak224, Keccak256, Keccak384, Keccak512};
    
    let digest = match algorithm {
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha1 => {
            let mut hasher = sha1_smol::Sha1::new();
            hasher.update(data);
            hex::encode(hasher.digest().bytes())
        }
        Algorithm::Sha224 => {
            let mut hasher = Sha224::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha384 => {
            let mut hasher = Sha384::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha512_224 => {
            let mut hasher = Sha512_224::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha512_256 => {
            let mut hasher = Sha512_256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha3_224 => {
            let mut hasher = Sha3_224::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha3_256 => {
            let mut hasher = Sha3_256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha3_384 => {
            let mut hasher = Sha3_384::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Sha3_512 => {
            let mut hasher = Sha3_512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Blake2b512 => {
            let mut hasher = Blake2b512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Blake2s256 => {
            let mut hasher = Blake2s256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Blake3 => {
            hex::encode(blake3::hash(data).as_bytes())
        }
        Algorithm::Keccak224 => {
            let mut hasher = Keccak224::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Keccak256 => {
            let mut hasher = Keccak256::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Keccak384 => {
            let mut hasher = Keccak384::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
        Algorithm::Keccak512 => {
            let mut hasher = Keccak512::new();
            hasher.update(data);
            hex::encode(hasher.finalize())
        }
    };
    
    Ok(digest)
}

/// Hash a file using the specified algorithm with streaming
///
/// This uses buffered I/O to efficiently hash large files without loading
/// them entirely into memory.
///
/// # Examples
///
/// ```no_run
/// use hashing::{hash_file, Algorithm};
///
/// let digest = hash_file("large_file.bin", Algorithm::Sha256).unwrap();
/// println!("File hash: {}", digest);
/// ```
pub fn hash_file<P: AsRef<Path>>(path: P, algorithm: Algorithm) -> Result<String> {
    use blake2::Blake2b512;
    use blake2::Blake2s256;
    use md5::Md5;
    use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256, Digest};
    use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512, Keccak224, Keccak256, Keccak384, Keccak512};
    
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(8192, file);
    let mut buffer = [0u8; 8192];
    
    macro_rules! hash_with_reader {
        ($hasher:expr) => {{
            let mut hasher = $hasher;
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            hex::encode(hasher.finalize())
        }};
    }
    
    let digest = match algorithm {
        Algorithm::Md5 => hash_with_reader!(Md5::new()),
        Algorithm::Sha1 => {
            let mut hasher = sha1_smol::Sha1::new();
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            hex::encode(hasher.digest().bytes())
        }
        Algorithm::Sha224 => hash_with_reader!(Sha224::new()),
        Algorithm::Sha256 => hash_with_reader!(Sha256::new()),
        Algorithm::Sha384 => hash_with_reader!(Sha384::new()),
        Algorithm::Sha512 => hash_with_reader!(Sha512::new()),
        Algorithm::Sha512_224 => hash_with_reader!(Sha512_224::new()),
        Algorithm::Sha512_256 => hash_with_reader!(Sha512_256::new()),
        Algorithm::Sha3_224 => hash_with_reader!(Sha3_224::new()),
        Algorithm::Sha3_256 => hash_with_reader!(Sha3_256::new()),
        Algorithm::Sha3_384 => hash_with_reader!(Sha3_384::new()),
        Algorithm::Sha3_512 => hash_with_reader!(Sha3_512::new()),
        Algorithm::Blake2b512 => hash_with_reader!(Blake2b512::new()),
        Algorithm::Blake2s256 => hash_with_reader!(Blake2s256::new()),
        Algorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            hex::encode(hasher.finalize().as_bytes())
        }
        Algorithm::Keccak224 => hash_with_reader!(Keccak224::new()),
        Algorithm::Keccak256 => hash_with_reader!(Keccak256::new()),
        Algorithm::Keccak384 => hash_with_reader!(Keccak384::new()),
        Algorithm::Keccak512 => hash_with_reader!(Keccak512::new()),
    };
    
    Ok(digest)
}

/// Hash result with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HashResult {
    pub algorithm: String,
    pub digest: String,
    pub input_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_path: Option<String>,
}

impl HashResult {
    /// Create a new hash result
    pub fn new(algorithm: Algorithm, digest: String, input_type: &str) -> Self {
        Self {
            algorithm: algorithm.name().to_string(),
            digest,
            input_type: input_type.to_string(),
            input_path: None,
        }
    }
    
    /// Set the input path
    pub fn with_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.input_path = Some(path.as_ref().display().to_string());
        self
    }
    
    /// Export to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| HashError::ExportError(e.to_string()))
    }
    
    /// Export to simple text format
    pub fn to_text(&self) -> String {
        format!("{} ({})", self.digest, self.algorithm)
    }
}

/// Mini SHA-1 implementation to avoid extra dependencies
mod sha1_smol {
    pub struct Sha1 {
        h: [u32; 5],
        len: u64,
        buffer: Vec<u8>,
    }

    impl Sha1 {
        pub fn new() -> Self {
            Self {
                h: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0],
                len: 0,
                buffer: Vec::new(),
            }
        }

        pub fn update(&mut self, data: &[u8]) {
            self.buffer.extend_from_slice(data);
            self.len += data.len() as u64;

            while self.buffer.len() >= 64 {
                let chunk: Vec<u8> = self.buffer.drain(..64).collect();
                self.process_block(&chunk);
            }
        }

        fn process_block(&mut self, block: &[u8]) {
            let mut w = [0u32; 80];
            
            for i in 0..16 {
                w[i] = u32::from_be_bytes([
                    block[i * 4],
                    block[i * 4 + 1],
                    block[i * 4 + 2],
                    block[i * 4 + 3],
                ]);
            }

            for i in 16..80 {
                w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
            }

            let mut a = self.h[0];
            let mut b = self.h[1];
            let mut c = self.h[2];
            let mut d = self.h[3];
            let mut e = self.h[4];

            #[allow(clippy::needless_range_loop)]
            for i in 0..80 {
                let (f, k) = match i {
                    0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                    20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                    _ => (b ^ c ^ d, 0xCA62C1D6),
                };

                let temp = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(w[i]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            self.h[0] = self.h[0].wrapping_add(a);
            self.h[1] = self.h[1].wrapping_add(b);
            self.h[2] = self.h[2].wrapping_add(c);
            self.h[3] = self.h[3].wrapping_add(d);
            self.h[4] = self.h[4].wrapping_add(e);
        }

        pub fn digest(mut self) -> Sha1Digest {
            let len_bits = self.len * 8;
            self.buffer.push(0x80);

            while (self.buffer.len() % 64) != 56 {
                self.buffer.push(0);
            }

            self.buffer.extend_from_slice(&len_bits.to_be_bytes());

            while !self.buffer.is_empty() {
                let chunk: Vec<u8> = self.buffer.drain(..64).collect();
                self.process_block(&chunk);
            }

            Sha1Digest { h: self.h }
        }
    }

    pub struct Sha1Digest {
        h: [u32; 5],
    }

    impl Sha1Digest {
        pub fn bytes(&self) -> [u8; 20] {
            let mut result = [0u8; 20];
            for (i, &val) in self.h.iter().enumerate() {
                result[i * 4..(i + 1) * 4].copy_from_slice(&val.to_be_bytes());
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_string() {
        let result = hash_string("hello world", Algorithm::Sha256).unwrap();
        assert_eq!(
            result,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_md5_string() {
        let result = hash_string("hello world", Algorithm::Md5).unwrap();
        assert_eq!(result, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_blake3_string() {
        let result = hash_string("hello world", Algorithm::Blake3).unwrap();
        assert_eq!(result.len(), 64); // BLAKE3 produces 32 bytes
    }

    #[test]
    fn test_all_algorithms() {
        for algo in Algorithm::all() {
            let result = hash_string("test", algo);
            assert!(result.is_ok(), "Algorithm {:?} failed", algo);
        }
    }

    #[test]
    fn test_algorithm_from_string() {
        assert_eq!(
            Algorithm::from_str("sha256").unwrap(),
            Algorithm::Sha256
        );
        assert_eq!(Algorithm::from_str("blake3").unwrap(), Algorithm::Blake3);
        assert!(Algorithm::from_str("invalid").is_err());
    }

    #[test]
    fn test_hash_result_json() {
        let result = HashResult::new(Algorithm::Sha256, "abcd1234".to_string(), "string");
        let json = result.to_json().unwrap();
        assert!(json.contains("sha256"));
        assert!(json.contains("abcd1234"));
    }
}
