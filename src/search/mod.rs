mod error;

use error::SearchError;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Semaphore;
use async_recursion::async_recursion;
use tokio::sync::mpsc;

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
            max_depth,
        }
    }

    /// Searches for files in the specified directory that match the given search pattern.
    pub async fn search_files_in_directory(
        &self,
        dir: PathBuf,
        search_pattern: String,
    ) -> Result<Vec<PathBuf>, SearchError> {
        let (tx, mut rx) = mpsc::channel(self.semaphore.available_permits() as usize);
        let _ = Self::find_files_recursively(dir, search_pattern, self.semaphore.clone(), 1, self.max_depth, tx).await?;
        let mut result = Vec::new();
        while let Some(path) = rx.recv().await {
            result.push(path);
        }
        Ok(result)
    }

    /// Recursively searches for files in the given directory that match the search pattern.
    #[async_recursion]
    async fn find_files_recursively(
        dir: PathBuf,
        search_pattern: String,
        semaphore: Arc<Semaphore>,
        current_depth: usize,
        max_depth: usize,
        tx: mpsc::Sender<PathBuf>,
    ) -> Result<(), SearchError> {
        let _permit = semaphore.acquire().await?;
        let mut entries = fs::read_dir(&dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
    
            if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
                if file_name.contains(&search_pattern) {
                    tx.send(path.clone()).await?;
                }
            }
    
            if path.is_dir() && current_depth < max_depth {
                let semaphore_clone = semaphore.clone();
                let search_pattern_clone = search_pattern.clone();
                let path_clone = path.clone();
                let tx_clone = tx.clone();
    
                let task = Self::find_files_recursively(path_clone, search_pattern_clone, semaphore_clone, current_depth + 1, max_depth, tx_clone);
                tokio::spawn(task);
            }
        }
    
        Ok(())
    }
    
}
