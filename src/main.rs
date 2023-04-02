use std::env;
use std::fs;
use std::io;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use tokio::task;
use tokio::sync::Semaphore;

async fn search_files_in_directory(dir: PathBuf, file_name_pattern: String, semaphore: Arc<Semaphore>) -> io::Result<Vec<PathBuf>> {
    let _permit = semaphore.acquire().await;
    let semaphore_clone = semaphore.clone();
    task::spawn_blocking(move || find_files_recursively(&dir, &file_name_pattern, semaphore_clone)).await.unwrap()
}

fn find_files_recursively(dir: &Path, file_name_pattern: &str, semaphore: Arc<Semaphore>) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
            if file_name.contains(file_name_pattern) {
                result.push(path.clone());
            }
        }

        if path.is_dir() {
            let found_files = find_files_recursively(path.as_path(), file_name_pattern, semaphore.clone())?;
            result.extend(found_files);
        }
    }

    Ok(result)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_name_pattern> [start_directory]", args[0]);
        println!("\nDescription:");
        println!("  This program searches for files with names containing the specified file_name_pattern.");
        println!("  If start_directory is not provided, the search starts from the current directory.");
        return;
    }

    let file_name_pattern = args[1].clone();
    let search_directory = PathBuf::from(if args.len() > 2 { &args[2] } else { "." });

    if !search_directory.is_dir() {
        eprintln!("Error: '{}' is not a directory.", search_directory.display());
        return;
    }

    let max_concurrent_threads = 4;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_threads));

    match search_files_in_directory(search_directory, file_name_pattern, semaphore).await {
        Ok(found_files) => {
            if found_files.is_empty() {
                println!("No files found.");
            } else {
                println!("Found {} files:", found_files.len());
                for path in found_files {
                    println!("{}", path.display());
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}