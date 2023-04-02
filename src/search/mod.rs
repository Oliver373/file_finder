use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

pub async fn search_files_in_directory(dir: PathBuf, file_name_pattern: String, semaphore: Arc<Semaphore>, max_depth: usize) -> io::Result<Vec<PathBuf>> {
    let _permit = semaphore.acquire().await;
    let semaphore_clone = semaphore.clone();
    task::spawn_blocking(move || find_files_recursively(&dir, &file_name_pattern, semaphore_clone,1, max_depth)).await.unwrap()
}

fn find_files_recursively(dir: &Path, file_name_pattern: &str, semaphore: Arc<Semaphore>, current_depth: usize, max_depth: usize) -> io::Result<Vec<PathBuf>> {
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
            let found_files = find_files_recursively(path.as_path(), file_name_pattern, semaphore.clone(), current_depth+1, max_depth)?;
            result.extend(found_files);
        }
    }

    Ok(result)
}