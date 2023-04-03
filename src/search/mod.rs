use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Semaphore;
use async_recursion::async_recursion;

/// `Search` struct is used to search for files in a directory that match a specific pattern.
pub struct Search {
    pub semaphore: Arc<Semaphore>,
    pub max_depth: usize,
}

impl Search {
    /// Creates a new `Search` instance with the specified maximum concurrent threads and maximum search depth.
    pub fn new(max_concurrent_threads: usize, max_depth: usize) -> Search {
        Search {
            semaphore: Arc::new(Semaphore::new(max_concurrent_threads)),
            max_depth: max_depth,
        }
    }

    /// Searches for files in the specified directory that match the given search pattern.
    pub async fn search_files_in_directory(
        &self,
        dir: PathBuf,
        search_pattern: String,
    ) -> io::Result<Vec<PathBuf>> {
        Self::find_files_recursively(dir, search_pattern, self.semaphore.clone(), 1, self.max_depth).await
    }

    /// Recursively searches for files in the given directory that match the search pattern.
    #[async_recursion]
    async fn find_files_recursively(
        dir: PathBuf,
        search_pattern: String,
        semaphore: Arc<Semaphore>,
        current_depth: usize,
        max_depth: usize,
    ) -> io::Result<Vec<PathBuf>> {
        let _permit = semaphore.acquire();
        let mut result = Vec::new();
        let mut entries = fs::read_dir(&dir).await?;

        let mut tasks = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
                if file_name.contains(&search_pattern) {
                    result.push(path.clone());
                }
            }

            if path.is_dir() && current_depth < max_depth {
                let semaphore_clone = semaphore.clone();
                let search_pattern_clone = search_pattern.clone();
                let path_clone = path.clone();

                let task = Self::find_files_recursively(path_clone, search_pattern_clone, semaphore_clone, current_depth + 1, max_depth);
                tasks.push(task);
            }
        }

        for task in tasks {
            let found_files = task.await?;
            result.extend_from_slice(&found_files);
        }

        Ok(result)
    }
}
