use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use tokio::task;

async fn search_files_in_directory(dir: PathBuf, search_name: String) -> io::Result<Vec<PathBuf>> {
    task::spawn_blocking(move || search_files_in_directory_sync(&dir, &search_name)).await.unwrap()
}

fn search_files_in_directory_sync(dir: &PathBuf, search_name: &str) -> io::Result<Vec<PathBuf>> {
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
            let found_files = search_files_in_directory_sync(&path, search_name)?;
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

    match search_files_in_directory(search_directory, search_pattern).await {
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