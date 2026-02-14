use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "find-duplicates")]
#[command(about = "Find duplicate images by comparing file size and SHA-256 checksum")]
struct Args {
    /// Path to the target image to find duplicates of
    target: PathBuf,

    /// Directory to search for duplicates
    search_dir: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show checksums in output
    #[arg(short = 'c', long)]
    show_checksums: bool,
}

fn calculate_sha256(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};

    let mut file = fs::File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn get_file_info(path: &Path) -> Result<(u64, String, String)> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("Failed to get metadata for: {}", path.display()))?;

    let size = metadata.len();
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let checksum = calculate_sha256(path)?;

    Ok((size, extension, checksum))
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get target file info
    if !args.target.exists() {
        anyhow::bail!("Target file does not exist: {}", args.target.display());
    }

    if !args.target.is_file() {
        anyhow::bail!("Target path is not a file: {}", args.target.display());
    }

    let (target_size, target_ext, target_checksum) = get_file_info(&args.target)?;

    if args.verbose {
        eprintln!("Target file: {}", args.target.display());
        eprintln!("  Size: {} bytes", target_size);
        eprintln!("  Extension: .{}", target_ext);
        eprintln!("  SHA-256: {}", target_checksum);
        eprintln!();
        eprintln!("Searching in: {}", args.search_dir.display());
        eprintln!();
    }

    let mut found_count = 0;
    let mut checked_count = 0;
    let mut size_matches = 0;

    // Track files by size for efficiency
    let mut files_by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    // First pass: collect files by size
    if args.verbose {
        eprintln!("Phase 1: Scanning directory for files...");
    }

    for entry in WalkDir::new(&args.search_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Skip the target file itself if it's in the search directory
        if path == args.target {
            continue;
        }

        // Check extension matches
        let ext = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        if ext != target_ext {
            continue;
        }

        // Get file size
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();
            files_by_size.entry(size).or_insert_with(Vec::new).push(path.to_path_buf());
        }
    }

    // Second pass: check checksums only for files with matching size
    if args.verbose {
        eprintln!("Phase 2: Checking checksums for size matches...");
        eprintln!();
    }

    if let Some(same_size_files) = files_by_size.get(&target_size) {
        size_matches = same_size_files.len();

        for path in same_size_files {
            checked_count += 1;

            if args.verbose {
                eprint!("Checking: {} ... ", path.display());
            }

            match calculate_sha256(path) {
                Ok(checksum) => {
                    if checksum == target_checksum {
                        found_count += 1;

                        if args.verbose {
                            eprintln!("MATCH!");
                        }

                        if args.show_checksums {
                            println!("{} [SHA-256: {}]", path.display(), checksum);
                        } else {
                            println!("{}", path.display());
                        }
                    } else if args.verbose {
                        eprintln!("different checksum");
                    }
                }
                Err(e) => {
                    if args.verbose {
                        eprintln!("ERROR: {}", e);
                    }
                }
            }
        }
    }

    if args.verbose {
        eprintln!();
        eprintln!("Summary:");
        eprintln!("  Files with matching size: {}", size_matches);
        eprintln!("  Checksums calculated: {}", checked_count);
        eprintln!("  Duplicates found: {}", found_count);
    }

    Ok(())
}