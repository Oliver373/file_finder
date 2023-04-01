use std::env;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <search_pattern> [start_directory]", args[0]);
        println!("\nDescription:");
        println!("  This program searches for files with names containing the specified search pattern.");
        println!("  If start_directory is not provided, the search starts from the current directory.");
        return;
    }

    let search_pattern = args[1].clone();
    let start_dir = if args.len() > 2 { args[2].clone() } else { ".".to_string() };

    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || search_files(&start_dir, &search_pattern, &tx));

    for result in rx {
        match result {
            Ok(path) => println!("{}", path),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn search_files<P: AsRef<Path>>(dir: P, search_pattern: &str,  tx: &mpsc::Sender<Result<String, std::io::Error>>) {
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(e) => {
            let _ = tx.send(Err(e));
            return;
        },
    };

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                let _ = tx.send(Err(e));
                continue;
            },
        };
        let path = entry.path();
        match path.file_name() {
            Some(file_name_os) => {
                let file_name = file_name_os.to_string_lossy();
                if file_name.contains(search_pattern) {
                    let _ = tx.send(Ok(path.to_string_lossy().to_string()));
                }
            },
            None => eprintln!("Failed to get file_name from entry_path: {}", path.display())
        }

        if path.is_dir() {
            let tx_clone = mpsc::Sender::clone(tx);
            let path_clone = path.clone();
            let search_pattern_clone = search_pattern.to_owned();
            let _ = thread::spawn(move || search_files(&path_clone, &search_pattern_clone, &tx_clone));
        }
        
    }

    if dir.as_ref() == Path::new(".") {
        drop(tx);
    }
}

