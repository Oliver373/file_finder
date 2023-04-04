use file_finder::search::Search;
use std::path::PathBuf;

#[tokio::test]
async fn test_integration_search_files_in_directory() {
    // Adjust the configuration parameters as needed for your test
    let search = Search::new(4, 5, true, false);

    // Specify the directory to search and the pattern to search for
    let start_directory = PathBuf::from(".");
    let search_pattern = "Cargo.toml"; // Replace with your desired search pattern

    let found_files = search
        .search_files_in_directory(start_directory.clone(), search_pattern)
        .await
        .expect("Failed to search for files");

    // Perform any assertions based on the found files
    assert!(
        found_files
            .iter()
            .any(|path| path == &start_directory.join("Cargo.toml")),
        "Expected file not found"
    );
}