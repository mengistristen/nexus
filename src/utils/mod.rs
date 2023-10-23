//! This module contains utilities used throughout the program.
use std::{fs, path::PathBuf};

/// Creates and returns the path for the application data directory.
///
/// # Returns
///
/// Returns the path to the data directory.
pub fn get_data_dir() -> PathBuf {
    let mut data_dir = dirs::data_local_dir().expect("failed to find data directory");

    data_dir.push("nexus");
    fs::create_dir_all(&data_dir).expect("failed to create data directory");

    data_dir
}

/// Searches the data directory and returns the names of all note files.
///
/// # Returns
///
/// Returns a list of the names of all notes.
pub fn get_note_names() -> Vec<String> {
    let data_dir = get_data_dir();
    let paths = fs::read_dir(data_dir).expect("failed to read data dir");
    let mut result = vec![];

    for path in paths {
        if let Ok(entry) = path {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str().expect("file name to convert to str");

            if file_name_str.ends_with(".md") {
                result.push(file_name_str.to_owned());
            }
        }
    }

    result
}
