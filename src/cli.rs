use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// The file name pattern to search for
    pub file_name_pattern: String,
    /// The start directory for the search, default_value="."
    pub start_directory: Option<String>,
}