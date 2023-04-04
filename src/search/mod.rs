mod error;
mod pattern;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Semaphore;
use tokio::sync::mpsc;
use async_recursion::async_recursion;

use self::error::SearchError;
use self::pattern::SearchPattern;

type PathBufSender = mpsc::Sender<PathBuf>;
// type PathBufReceiver = mpsc::Receiver<PathBuf>;

/// `Search` struct is used to search for files in a directory that match a specific pattern.
pub struct Search {
    // Inner state shared between instances and async tasks.
    inner: Arc<Inner>,
}

// Holds shared state for the `Search` struct.
struct Inner {
    // Controls the maximum number of concurrent tasks.
    semaphore: Semaphore,
    // Limits the search depth when searching directories recursively.
    max_depth: usize,
    // Determines whether to use a semaphore to control concurrency.
    use_semaphore: bool,
    // Determines whether to use regex for the search pattern.
    use_regex: bool,
}

impl Search {
    /// Creates a new `Search` instance with the specified maximum concurrent threads and maximum search depth.
    pub fn new(max_concurrent_threads: usize, max_depth: usize, use_semaphore: bool, use_regex: bool) -> Search {
        Search {
            inner: Arc::new(Inner {
                semaphore: Semaphore::new(max_concurrent_threads),
                max_depth,
                use_semaphore,
                use_regex,
            }),
        }
    }

    /// Searches for files in the specified directory that match the given search pattern.
    pub async fn search_files_in_directory(
        &self,
        dir: PathBuf,
        search_pattern: impl Into<String>,
    ) -> Result<Vec<PathBuf>, SearchError> {
        let (tx, mut rx) = mpsc::channel(self.inner.semaphore.available_permits() as usize);
        let pattern = SearchPattern::new(self.inner.use_regex, search_pattern)?;
        Self::find_files_recursively(self.inner.clone(), dir, pattern, 1, tx).await?;
        let mut result = Vec::new();
        while let Some(path) = rx.recv().await {
            result.push(path);
        }
        Ok(result)
    }

    /// Recursively searches for files in the given directory that match the search pattern.
    #[async_recursion]
    async fn find_files_recursively(
        inner: Arc<Inner>,
        dir: PathBuf,
        search_pattern: SearchPattern,
        current_depth: usize,
        tx: PathBufSender,
    ) -> Result<(), SearchError> {
        if inner.use_semaphore {
            let _permit = inner.semaphore.acquire().await?;
        }
        let mut entries = fs::read_dir(&dir).await?;
    
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
    
            if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
                if search_pattern.is_match(file_name) {
                    tx.send(path.clone()).await?;
                }
            }
    
            if path.is_dir() && current_depth < inner.max_depth {
                let search_pattern_clone = search_pattern.clone();
                let path_clone = path.clone();
                let tx_clone = tx.clone();
    
                let task = Self::find_files_recursively(
                    inner.clone(),
                    path_clone,
                    search_pattern_clone,
                    current_depth + 1,
                    tx_clone,
                );
                tokio::spawn(task);
            }
        }
    
        Ok(())
    }
    
}
