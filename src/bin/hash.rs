// File: src\bin\hash.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2026-02-15
// Description: Command-line interface for the hashing tool
// License: MIT

//! Command-line interface for the hashing tool

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use hashing::{hash_file, hash_string, Algorithm, HashResult};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use clap_version_flag::colorful_version;

#[derive(Parser)]
#[command(name = "hash")]
#[command(author = "hadi cahyadi <cumulus13@gmail.com>")]
#[command(version)]
#[command(about = "Generate cryptographic hashes for strings and files", long_about = "A Rust library and CLI tool for generating cryptographic hashes. Supports multiple algorithms with zero-copy streaming for efficient processing of large files.")]
struct Cli {
    /// Input: file path or string to hash
    #[arg(value_name = "INPUT")]
    input: String,

    /// Hash algorithm to use
    #[arg(short, long, default_value = "sha256")]
    algorithm: String,

    /// List all available algorithms
    #[arg(short = 'l', long)]
    list_algorithms: bool,

    /// Export result to file
    #[arg(short, long, value_name = "FILE")]
    export: Option<PathBuf>,

    /// Export format
    #[arg(short = 'f', long, default_value = "text")]
    format: ExportFormat,

    /// Compute hashes for all algorithms
    #[arg(short = 'A', long)]
    all_algorithms: bool,

    /// Treat input as a string even if it matches a file path
    #[arg(short = 's', long)]
    string: bool,

    /// Quiet mode - only output the hash
    #[arg(short, long)]
    quiet: bool,

    /// Verify hash against expected value
    #[arg(short = 'c', long, value_name = "EXPECTED")]
    verify: Option<String>,

    /// Compare two files or strings by hash
    #[arg(short = 'C', long, value_name = "INPUT2")]
    compare: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum ExportFormat {
    /// Plain text format (hash only)
    Text,
    /// JSON format with metadata
    Json,
    /// Checksums format (hash filename)
    Checksum,
}

fn main() -> Result<()> {
    let os_args: Vec<String> = std::env::args().collect();
    let version_flags = ["-V", "--version"];
    let list_flags = ["-l", "--list-algorithms"];

    if os_args.len() >= 2 {
        if os_args[1..].iter().any(|arg| version_flags.contains(&arg.as_str())) {
            let version = colorful_version!();
            version.print_and_exit();    
        } else if os_args[1..].iter().any(|arg| list_flags.contains(&arg.as_str())) {
            list_algorithms();
            return Ok(());
        }
        
    }

    let cli = Cli::parse();

    // List algorithms if requested
    if cli.list_algorithms {
        list_algorithms();
        return Ok(());
    }

    // Compare mode if requested
    if let Some(ref input2) = cli.compare {
        return compare_inputs(&cli.input, input2, &cli)?;
    }

    // Process input
    if cli.all_algorithms {
        process_all_algorithms(&cli)?;
    } else {
        process_single_algorithm(&cli)?;
    }

    Ok(())
}

fn list_algorithms() {
    println!("Available hash algorithms:");
    println!();
    
    let algorithms = vec![
        ("MD5", "md5", "128-bit (insecure, legacy use only)"),
        ("SHA-1", "sha1", "160-bit (insecure, legacy use only)"),
        ("SHA-224", "sha224", "224-bit SHA-2"),
        ("SHA-256", "sha256", "256-bit SHA-2 (recommended)"),
        ("SHA-384", "sha384", "384-bit SHA-2"),
        ("SHA-512", "sha512", "512-bit SHA-2"),
        ("SHA-512/224", "sha512-224", "224-bit SHA-2 variant"),
        ("SHA-512/256", "sha512-256", "256-bit SHA-2 variant"),
        ("SHA3-224", "sha3-224", "224-bit SHA-3"),
        ("SHA3-256", "sha3-256", "256-bit SHA-3"),
        ("SHA3-384", "sha3-384", "384-bit SHA-3"),
        ("SHA3-512", "sha3-512", "512-bit SHA-3"),
        ("BLAKE2b", "blake2b", "512-bit BLAKE2b"),
        ("BLAKE2s", "blake2s", "256-bit BLAKE2s"),
        ("BLAKE3", "blake3", "256-bit BLAKE3 (fast, modern)"),
        ("Keccak-224", "keccak224", "224-bit Keccak"),
        ("Keccak-256", "keccak256", "256-bit Keccak"),
        ("Keccak-384", "keccak384", "384-bit Keccak"),
        ("Keccak-512", "keccak512", "512-bit Keccak"),
    ];

    for (name, code, desc) in algorithms {
        println!("  {:<15} {:<15} {}", name, code, desc);
    }
}

fn process_single_algorithm(cli: &Cli) -> Result<()> {
    let algorithm = Algorithm::from_str(&cli.algorithm)
        .with_context(|| format!("Invalid algorithm: {}", cli.algorithm))?;

    let (digest, input_type, input_path) = compute_hash(&cli.input, algorithm, cli.string)?;

    // Verify if requested
    if let Some(expected) = &cli.verify {
        let matches = digest.eq_ignore_ascii_case(expected.trim());
        if cli.quiet {
            std::process::exit(if matches { 0 } else { 1 });
        } else if matches {
            println!("✓ Hash verification PASSED");
            println!("{}: {}", algorithm.name().to_uppercase(), digest);
        } else {
            eprintln!("✗ Hash verification FAILED");
            eprintln!("Expected: {}", expected);
            eprintln!("Got:      {}", digest);
            std::process::exit(1);
        }
        return Ok(());
    }

    // Display result
    if cli.quiet {
        println!("{}", digest);
    } else {
        display_result(algorithm, &digest, &input_type, input_path.as_deref());
    }

    // Export if requested
    if let Some(export_path) = &cli.export {
        let mut result = HashResult::new(algorithm, digest.clone(), &input_type);
        if let Some(path) = input_path {
            result = result.with_path(path);
        }
        export_result(&result, export_path, &cli.format)?;
    }

    Ok(())
}

fn process_all_algorithms(cli: &Cli) -> Result<()> {
    let mut results = Vec::new();

    if !cli.quiet {
        println!("Computing hashes for all algorithms...");
        println!();
    }

    for algorithm in Algorithm::all() {
        let (digest, input_type, input_path) = compute_hash(&cli.input, algorithm, cli.string)?;
        
        if !cli.quiet {
            println!("{:<15} {}", format!("{}:", algorithm.name().to_uppercase()), digest);
        }

        let mut result = HashResult::new(algorithm, digest, &input_type);
        if let Some(ref path) = input_path {
            result = result.with_path(path);
        }
        results.push(result);
    }

    // Export if requested
    if let Some(export_path) = &cli.export {
        export_all_results(&results, export_path, &cli.format)?;
    }

    Ok(())
}

fn compute_hash(
    input: &str,
    algorithm: Algorithm,
    force_string: bool,
) -> Result<(String, String, Option<String>)> {
    // Check if input is a file path (unless forced to treat as string)
    if !force_string && Path::new(input).exists() {
        let digest = hash_file(input, algorithm)
            .with_context(|| format!("Failed to hash file: {}", input))?;
        Ok((digest, "file".to_string(), Some(input.to_string())))
    } else {
        let digest = hash_string(input, algorithm)
            .with_context(|| "Failed to hash string")?;
        Ok((digest, "string".to_string(), None))
    }
}

fn display_result(algorithm: Algorithm, digest: &str, input_type: &str, input_path: Option<&str>) {
    println!("Algorithm:  {}", algorithm.name().to_uppercase());
    println!("Input type: {}", input_type);
    if let Some(path) = input_path {
        println!("File path:  {}", path);
    }
    println!("Hash:       {}", digest);
}

fn export_result(result: &HashResult, path: &Path, format: &ExportFormat) -> Result<()> {
    let content = match format {
        ExportFormat::Text => result.digest.clone(),
        ExportFormat::Json => result.to_json()?,
        ExportFormat::Checksum => {
            if let Some(ref file_path) = result.input_path {
                format!("{}  {}", result.digest, file_path)
            } else {
                result.digest.clone()
            }
        }
    };

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    // Write to file
    fs::write(path, content.as_bytes())
        .with_context(|| format!("Failed to write to file: {}", path.display()))?;

    println!("Exported to: {}", path.display());
    Ok(())
}

fn export_all_results(results: &[HashResult], base_path: &Path, format: &ExportFormat) -> Result<()> {
    match format {
        ExportFormat::Json => {
            let json = serde_json::to_string_pretty(results)
                .context("Failed to serialize results to JSON")?;
            fs::write(base_path, json.as_bytes())
                .with_context(|| format!("Failed to write to file: {}", base_path.display()))?;
            println!("Exported all results to: {}", base_path.display());
        }
        ExportFormat::Text | ExportFormat::Checksum => {
            let parent = base_path.parent().unwrap_or_else(|| Path::new("."));
            let stem = base_path.file_stem().unwrap_or_default().to_string_lossy();
            let ext = base_path.extension().unwrap_or_default().to_string_lossy();

            for result in results {
                let filename = if ext.is_empty() {
                    format!("{}.{}", stem, result.algorithm)
                } else {
                    format!("{}.{}.{}", stem, result.algorithm, ext)
                };
                let file_path = parent.join(filename);

                let content = match format {
                    ExportFormat::Text => result.digest.clone(),
                    ExportFormat::Checksum => {
                        if let Some(ref path) = result.input_path {
                            format!("{}  {}", result.digest, path)
                        } else {
                            result.digest.clone()
                        }
                    }
                    _ => unreachable!(),
                };

                fs::write(&file_path, content.as_bytes())
                    .with_context(|| format!("Failed to write to file: {}", file_path.display()))?;
            }
            println!("Exported all results to: {}.*", base_path.display());
        }
    }

    Ok(())
}

fn compare_inputs(input1: &str, input2: &str, cli: &Cli) -> Result<()> {
    if cli.all_algorithms {
        compare_all_algorithms(input1, input2, cli)
    } else {
        compare_single_algorithm(input1, input2, cli)
    }
}

fn compare_single_algorithm(input1: &str, input2: &str, cli: &Cli) -> Result<()> {
    let algorithm = Algorithm::from_str(&cli.algorithm)
        .with_context(|| format!("Invalid algorithm: {}", cli.algorithm))?;

    let (hash1, type1, path1) = compute_hash(input1, algorithm, cli.string)?;
    let (hash2, type2, path2) = compute_hash(input2, algorithm, cli.string)?;

    let matches = hash1 == hash2;

    if cli.quiet {
        std::process::exit(if matches { 0 } else { 1 });
    }

    // Display comparison results
    println!("Comparing using {}", algorithm.name().to_uppercase());
    println!();
    println!("Input 1: {} ({})", path1.as_deref().unwrap_or(input1), type1);
    println!("Hash 1:  {}", hash1);
    println!();
    println!("Input 2: {} ({})", path2.as_deref().unwrap_or(input2), type2);
    println!("Hash 2:  {}", hash2);
    println!();

    if matches {
        println!("✓ MATCH - Inputs are identical");
        Ok(())
    } else {
        println!("✗ NO MATCH - Inputs are different");
        std::process::exit(1);
    }
}

fn compare_all_algorithms(input1: &str, input2: &str, cli: &Cli) -> Result<()> {
    let mut all_match = true;
    let mut match_count = 0;
    let mut mismatch_count = 0;

    if !cli.quiet {
        println!("Comparing with all algorithms...");
        println!();
    }

    for algorithm in Algorithm::all() {
        let (hash1, _, _) = compute_hash(input1, algorithm, cli.string)?;
        let (hash2, _, _) = compute_hash(input2, algorithm, cli.string)?;

        let matches = hash1 == hash2;
        
        if matches {
            match_count += 1;
        } else {
            mismatch_count += 1;
            all_match = false;
        }

        if !cli.quiet {
            let status = if matches { "✓" } else { "✗" };
            println!("{} {:<15} {} | {}", 
                status,
                format!("{}:", algorithm.name().to_uppercase()),
                if matches { "MATCH" } else { "DIFFERENT" },
                if matches { "" } else { &format!("{} ≠ {}", &hash1[..16], &hash2[..16]) }
            );
        }
    }

    if !cli.quiet {
        println!();
        println!("Results: {} matches, {} mismatches", match_count, mismatch_count);
        
        if all_match {
            println!("✓ ALL ALGORITHMS MATCH - Inputs are identical");
        } else {
            println!("✗ MISMATCHES DETECTED - Inputs are different");
        }
    }

    if all_match {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash_string() {
        let (digest, input_type, path) = compute_hash("test", Algorithm::Sha256, true).unwrap();
        assert_eq!(input_type, "string");
        assert!(path.is_none());
        assert_eq!(digest.len(), 64);
    }
}
