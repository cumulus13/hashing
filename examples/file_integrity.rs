use hashing::{hash_file, Algorithm, HashResult};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== File Integrity Verification Example ===\n");

    // Create a temporary test file
    let test_file = "test_document.txt";
    let original_content = "This is an important document that must maintain integrity.";
    
    println!("Creating test file: {}", test_file);
    fs::write(test_file, original_content)?;

    // Step 1: Calculate initial hash
    println!("\n1. Calculate initial hash:");
    let original_hash = hash_file(test_file, Algorithm::Sha256)?;
    println!("   Original SHA-256: {}", original_hash);

    // Step 2: Create hash result with metadata
    let hash_record = HashResult::new(
        Algorithm::Sha256,
        original_hash.clone(),
        "file"
    ).with_path(test_file);

    // Save the hash for verification
    let hash_file_path = "test_document.sha256";
    fs::write(hash_file_path, hash_record.to_json()?)?;
    println!("   Hash saved to: {}", hash_file_path);

    // Step 3: Simulate file modification
    println!("\n2. Simulating file modification...");
    let modified_content = "This document has been tampered with!";
    fs::write(test_file, modified_content)?;
    println!("   File modified");

    // Step 4: Verify integrity
    println!("\n3. Verify file integrity:");
    let current_hash = hash_file(test_file, Algorithm::Sha256)?;
    
    if current_hash == original_hash {
        println!("   ✓ File integrity VERIFIED - file unchanged");
    } else {
        println!("   ✗ File integrity FAILED - file has been modified!");
        println!("   Expected: {}", original_hash);
        println!("   Got:      {}", current_hash);
    }

    // Step 5: Restore and re-verify
    println!("\n4. Restore original content and re-verify:");
    fs::write(test_file, original_content)?;
    let restored_hash = hash_file(test_file, Algorithm::Sha256)?;
    
    if restored_hash == original_hash {
        println!("   ✓ File integrity VERIFIED - file restored");
        println!("   Hash: {}", restored_hash);
    } else {
        println!("   ✗ Unexpected hash mismatch");
    }

    // Step 6: Multiple algorithm verification
    println!("\n5. Multi-algorithm verification:");
    let algorithms = vec![
        Algorithm::Sha256,
        Algorithm::Sha512,
        Algorithm::Blake3,
    ];

    for algo in algorithms {
        let hash = hash_file(test_file, algo)?;
        println!("   {:<10} {}", format!("{}:", algo.name()), hash);
    }

    // Clean up
    println!("\n6. Cleaning up test files...");
    fs::remove_file(test_file)?;
    fs::remove_file(hash_file_path)?;
    println!("   Test files removed");

    println!("\n=== Example Complete ===");
    Ok(())
}
