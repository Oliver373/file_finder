use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <search_pattern> [start_directory]", args[0]);
        println!("\nDescription:");
        println!("  This program searches for files with names containing the specified search pattern.");
        println!("  If start_directory is not provided, the search starts from the current directory.");
        return;
    }

    let search_pattern = &args[1];
    let start_dir = if args.len() > 2 { &args[2] } else { "." };

    match search_files(start_dir, search_pattern) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn search_files<P: AsRef<Path>>(dir: P, search_pattern: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(dir)?;

    for entry_result in entries {
        let entry = entry_result?;
        let path = entry.path();
        match path.file_name() {
            Some(file_name_os) => {
                let file_name = file_name_os.to_string_lossy();
                if file_name.contains(search_pattern) {
                    println!("{}", path.display());
                }
            },
            None => eprintln!("Failed to get file_name from entry_path: {}", path.display())
        }

        if path.is_dir() {
            search_files(&path, search_pattern)?;
        }
    }

    Ok(())
}

