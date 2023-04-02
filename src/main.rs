use std::env;
use std::fs;
use std::io;
use std::sync::Arc;
use std::path::PathBuf;
use tokio::task;
use tokio::sync::Semaphore;

async fn search_files_in_directory(dir: PathBuf, search_name: String, semaphore: Arc<Semaphore>) -> io::Result<Vec<PathBuf>> {
    let _permit = semaphore.acquire().await;
    let semaphore_clone = semaphore.clone();
    task::spawn_blocking(move || search_files_in_directory_sync(&dir, &search_name, semaphore_clone)).await.unwrap()
}

fn search_files_in_directory_sync(dir: &PathBuf, search_name: &str, semaphore: Arc<Semaphore>) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();

        if file_name.to_string_lossy().contains(search_name) {
            result.push(path.clone());
        }

        if path.is_dir() {
            let found_files = search_files_in_directory_sync(&path, search_name, semaphore.clone())?;
            result.extend(found_files);
        }
    }

    Ok(result)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <search_pattern> [start_directory]", args[0]);
        println!("\nDescription:");
        println!("  This program searches for files with names containing the specified search pattern.");
        println!("  If start_directory is not provided, the search starts from the current directory.");
        return;
    }

    let search_pattern = args[1].clone();
    let search_directory = PathBuf::from(if args.len() > 2 { &args[2] } else { "." });

    if !search_directory.is_dir() {
        eprintln!("Error: '{}' is not a directory.", search_directory.display());
        return;
    }

    let max_concurrent_threads = 4;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_threads));

    match search_files_in_directory(search_directory, search_pattern, semaphore).await {
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