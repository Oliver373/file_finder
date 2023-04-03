use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub use_semaphore: bool,
    pub max_concurrent_threads: usize,
    pub max_depth: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            use_semaphore: false,
            max_concurrent_threads: 4,
            max_depth: 4,
        }
    }
}