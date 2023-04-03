use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub enable_semaphore: bool,
    pub max_concurrent_threads: usize,
    pub max_depth: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enable_semaphore: false,
            max_concurrent_threads: 4,
            max_depth: 4,
        }
    }
}