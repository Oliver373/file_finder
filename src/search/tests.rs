use super::*;
use super::error::SearchError;
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;

#[tokio::test]
async fn test_search_files_in_directory() {
    // Create a temporary directory with test files and subdirectories.
    let temp_dir = TempDir::new("search_test").unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let temp_subdir = temp_path.join("subdir");
    std::fs::create_dir(&temp_subdir).unwrap();

    let file1 = temp_path.join("file1.txt");
    let mut file1_handle = File::create(&file1).unwrap();
    file1_handle.write_all(b"file1 content").unwrap();

    let file2 = temp_subdir.join("file2.txt");
    let mut file2_handle = File::create(&file2).unwrap();
    file2_handle.write_all(b"file2 content").unwrap();

    let search = Search::new(4, 2, true, false);
    let pattern = "file";
    let results = search.search_files_in_directory(temp_path.clone(), pattern.to_string()).await.unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.contains(&file1));
    assert!(results.contains(&file2));
}

#[tokio::test]
async fn test_search_files_with_regex() {
    let temp_dir = TempDir::new("search_test_regex").unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let temp_subdir = temp_path.join("subdir");
    std::fs::create_dir(&temp_subdir).unwrap();

    let file1 = temp_path.join("fileA.txt");
    let mut file1_handle = File::create(&file1).unwrap();
    file1_handle.write_all(b"fileA content").unwrap();

    let file2 = temp_subdir.join("fileB.txt");
    let mut file2_handle = File::create(&file2).unwrap();
    file2_handle.write_all(b"fileB content").unwrap();

    let search = Search::new(4, 2, true, true);
    let pattern = r"file[A|B]\.txt";
    let results = search.search_files_in_directory(temp_path.clone(), pattern.to_string()).await.unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.contains(&file1));
    assert!(results.contains(&file2));
}

#[tokio::test]
async fn test_search_files_max_depth() {
    let temp_dir = TempDir::new("search_test_max_depth").unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let temp_subdir = temp_path.join("subdir");
    std::fs::create_dir(&temp_subdir).unwrap();

    let file1 = temp_path.join("file1.txt");
    let mut file1_handle = File::create(&file1).unwrap();
    file1_handle.write_all(b"file1 content").unwrap();

    let file2 = temp_subdir.join("file2.txt");
    let mut file2_handle = File::create(&file2).unwrap();
    file2_handle.write_all(b"file2 content").unwrap();

    let search = Search::new(4, 1, true, false);
    let pattern = "file";
    let results = search.search_files_in_directory(temp_path.clone(), pattern.to_string()).await.unwrap();

    assert_eq!(results.len(), 1);
    assert!(results.contains(&file1));
    assert!(!results.contains(&file2));
}

#[tokio::test]
async fn test_search_files_no_match() {
    let temp_dir = TempDir::new("search_test_no_match").unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    let file1 = temp_path.join("file1.txt");
    let mut file1_handle = File::create(&file1).unwrap();
    file1_handle.write_all(b"file1 content").unwrap();

    let search = Search::new(4, 2, true, false);
    let pattern = "no_match*.txt";
    let results = search.search_files_in_directory(temp_path.clone(), pattern.to_string()).await.unwrap();

    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_search_error_invalid_regex() {
    let temp_dir = TempDir::new("search_test_invalid_regex").unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    let search = Search::new(4, 2, true, true);
    let pattern = r"file[A|B\.txt"; // Invalid regex, missing closing ']'
    let result = search.search_files_in_directory(temp_path.clone(), pattern.to_string()).await;

    assert!(result.is_err());
    if let Err(err) = result {
        match err {
            SearchError::RegexError(_) => (),
            _ => panic!("Expected a RegexError"),
        }
    } else {
        panic!("Expected an error");
    }
}

