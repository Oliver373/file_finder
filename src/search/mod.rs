use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::{runtime::Handle, task};

pub async fn search_files_in_directory(
    dir: PathBuf,
    file_name_pattern: String,
    semaphore: Arc<Semaphore>,
    max_depth: usize,
) -> io::Result<Vec<PathBuf>> {
    let _permit = semaphore.acquire().await;
    let semaphore_clone = semaphore.clone();
    task::spawn_blocking(move || find_files_recursively(dir, file_name_pattern, semaphore_clone, 1, max_depth))
        .await
        .unwrap()
}

fn find_files_recursively(
    dir: PathBuf,
    file_name_pattern: String,
    semaphore: Arc<Semaphore>,
    current_depth: usize,
    max_depth: usize,
) -> io::Result<Vec<PathBuf>> {
    let _permit = semaphore.acquire();
    let mut result = Vec::new();
    let entries = fs::read_dir(&dir)?;

    let mut tasks = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
            if file_name.contains(&file_name_pattern) {
                result.push(path.clone());
            }
        }

        if path.is_dir() && current_depth < max_depth {
            let semaphore_clone = semaphore.clone();
            let file_name_pattern = file_name_pattern.clone();
            let path_clone = path.clone();

            let task = task::spawn_blocking(move || {
                find_files_recursively(path_clone, file_name_pattern, semaphore_clone, current_depth + 1, max_depth)
            });

            tasks.push(task);
        }
    }

    for task in tasks {
        let found_files = Handle::current().block_on(task)??;
        result.extend_from_slice(&found_files);
    }

    Ok(result)
}
