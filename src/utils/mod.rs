use std::{fs, path::PathBuf};

/// Creates and returns the path for the application data directory.
pub fn get_data_dir() -> PathBuf {
    let mut data_dir = dirs::data_local_dir().expect("failed to find data directory");

    data_dir.push("nexus");
    fs::create_dir_all(&data_dir).expect("failed to create data directory");

    data_dir
}
