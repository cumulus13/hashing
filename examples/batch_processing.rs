use hashing::{hash_file, Algorithm, HashResult};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Batch File Processing Example ===\n");

    // Create sample files
    let test_files = vec![
        ("file1.txt", "Content of file 1"),
        ("file2.txt", "Content of file 2"),
        ("file3.txt", "Content of file 3"),
    ];

    println!("Creating test files...");
    for (filename, content) in &test_files {
        fs::write(filename, content)?;
        println!("  Created: {}", filename);
    }

    // Process all files with multiple algorithms
    println!("\nProcessing files with multiple algorithms...\n");
    
    let algorithms = vec![
        Algorithm::Sha256,
        Algorithm::Blake3,
    ];

    let mut results = Vec::new();

    for (filename, _) in &test_files {
        println!("File: {}", filename);
        
        for algo in &algorithms {
            let hash = hash_file(filename, *algo)?;
            println!("  {:<10} {}", format!("{}:", algo.name()), hash);
            
            let result = HashResult::new(*algo, hash, "file").with_path(filename);
            results.push(result);
        }
        println!();
    }

    // Export all results to JSON
    println!("Exporting results...");
    let json_output = serde_json::to_string_pretty(&results)?;
    let output_file = "batch_results.json";
    fs::write(output_file, json_output)?;
    println!("  Results saved to: {}", output_file);

    // Create checksum files (like sha256sum format)
    println!("\nCreating checksum files...");
    for algo in &algorithms {
        let checksum_file = format!("checksums.{}", algo.name());
        let mut content = String::new();
        
        for (filename, _) in &test_files {
            let hash = hash_file(filename, *algo)?;
            content.push_str(&format!("{}  {}\n", hash, filename));
        }
        
        fs::write(&checksum_file, content)?;
        println!("  Created: {}", checksum_file);
    }

    // Verify checksums
    println!("\nVerifying checksums from file...");
    let checksum_content = fs::read_to_string("checksums.sha256")?;
    
    for line in checksum_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let (expected_hash, filename) = (parts[0], parts[1]);
            let actual_hash = hash_file(filename, Algorithm::Sha256)?;
            
            if actual_hash == expected_hash {
                println!("  ✓ {} - verified", filename);
            } else {
                println!("  ✗ {} - FAILED", filename);
            }
        }
    }

    // Clean up
    println!("\nCleaning up...");
    for (filename, _) in &test_files {
        fs::remove_file(filename)?;
    }
    fs::remove_file(output_file)?;
    for algo in &algorithms {
        fs::remove_file(format!("checksums.{}", algo.name()))?;
    }
    println!("  All test files removed");

    println!("\n=== Example Complete ===");
    Ok(())
}
