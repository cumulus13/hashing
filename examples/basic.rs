use hashing::{hash_string, hash_bytes, Algorithm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hashing Library - Basic Examples ===\n");

    // Example 1: Hash a simple string
    println!("1. Hash a string:");
    let message = "Hello, World!";
    let sha256_hash = hash_string(message, Algorithm::Sha256)?;
    println!("   Input:  {}", message);
    println!("   SHA256: {}\n", sha256_hash);

    // Example 2: Hash with different algorithms
    println!("2. Same input, different algorithms:");
    let input = "test data";
    
    let algorithms = vec![
        Algorithm::Md5,
        Algorithm::Sha256,
        Algorithm::Blake3,
    ];

    for algo in algorithms {
        let hash = hash_string(input, algo)?;
        println!("   {:<10} {}", format!("{}:", algo.name()), hash);
    }
    println!();

    // Example 3: Hash binary data
    println!("3. Hash binary data:");
    let binary_data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello" in hex
    let hash = hash_bytes(&binary_data, Algorithm::Sha256)?;
    println!("   Binary data: {:?}", binary_data);
    println!("   SHA256:      {}\n", hash);

    // Example 4: Compare different SHA-2 variants
    println!("4. SHA-2 family comparison:");
    let data = "Compare SHA-2 variants";
    
    let sha2_variants = vec![
        Algorithm::Sha224,
        Algorithm::Sha256,
        Algorithm::Sha384,
        Algorithm::Sha512,
    ];

    for algo in sha2_variants {
        let hash = hash_string(data, algo)?;
        println!("   {:<12} {} (length: {})", 
                 format!("{}:", algo.name()), 
                 &hash[..32], // Show first 32 chars
                 hash.len());
    }
    println!();

    // Example 5: Verify hash consistency
    println!("5. Hash consistency check:");
    let test_input = "consistency test";
    let hash1 = hash_string(test_input, Algorithm::Sha256)?;
    let hash2 = hash_string(test_input, Algorithm::Sha256)?;
    println!("   Hash 1: {}", hash1);
    println!("   Hash 2: {}", hash2);
    println!("   Match:  {}\n", hash1 == hash2);

    // Example 6: Performance comparison
    println!("6. Quick performance test:");
    let test_data = "x".repeat(10000);
    
    let start = std::time::Instant::now();
    hash_string(&test_data, Algorithm::Blake3)?;
    let blake3_time = start.elapsed();
    
    let start = std::time::Instant::now();
    hash_string(&test_data, Algorithm::Sha256)?;
    let sha256_time = start.elapsed();
    
    println!("   BLAKE3:  {:?}", blake3_time);
    println!("   SHA256:  {:?}", sha256_time);

    Ok(())
}
