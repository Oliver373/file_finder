use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub open_concurrent_threads_number_control: bool,
    pub max_concurrent_threads: usize,
    pub max_depth: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            open_concurrent_threads_number_control: false,
            max_concurrent_threads: 4,
            max_depth: 4,
        }
    }
}