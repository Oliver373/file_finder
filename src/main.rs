mod cli;
mod config;
mod search;

use cli::Cli;
use config::Config;
use search::Search;

use std::env;
use std::path::PathBuf;
use std::error::Error;
use clap::Parser;

async fn run(args: Cli, config: Config) -> Result<(), Box<dyn Error>> {
    let start_directory = PathBuf::from(args.start_directory.unwrap_or_else(|| ".".to_string()));

    if !start_directory.is_dir() {
        return Err(format!("Failed to start search: '{}' is not a directory.", start_directory.display()).into());
    }
    let search = Search::new(config.max_concurrent_threads, config.max_depth, config.use_semaphore, args.regex);

    match search.search_files_in_directory(start_directory, args.file_name_pattern).await {
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or("Failed to get the directory of the executable")?;
    let config_file = exe_dir.join("config.toml");
    let config: Config = confy::load_path(&config_file)?;
    println!("Config: {:?}", config);
    println!("args: {:?}", args);

    run(args, config).await?;

    Ok(())
}