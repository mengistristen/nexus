use serde::{Deserialize, Serialize};
use sha1::Digest;
use std::{
    env, fs,
    io::{Read, Seek, SeekFrom, Write},
    process::Command,
};
use tempfile::NamedTempFile;

use crate::args::{Args, Commands, Parser};

pub mod args;

/// The metadata associated with a note.
#[derive(Serialize, Deserialize)]
struct Metadata {
    /// The user-specified name for the note.
    name: String,
    /// The hash of the contents of the note.
    hash: String,
    /// A list of branch names and pointers to previous notes.
    prev: Vec<(String, String)>,
}

/// Creates a temporary file and spawns an editor to allow the user to write
/// their note.
fn create_user_note() -> NamedTempFile {
    // have the user write to a temporary file
    let file = NamedTempFile::new().expect("failed to create temporary file");

    // use whichever editor the user prefers, default to vim
    let editor = env::var("EDITOR").unwrap_or("vim".to_owned());

    // spawn a process to allow the user to write their note;
    // we'll add metadata later
    Command::new(editor)
        .arg(file.path().as_os_str())
        .status()
        .expect("failed to spawn editor");

    file
}

/// Reads the user's note from a temporary file.
fn get_user_content(mut file: NamedTempFile) -> String {
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    buffer
}

/// Hashes the note contents and creates the metadata for a note.
fn create_metadata(name: String, content: &String) -> Metadata {
    let mut hasher = sha1::Sha1::new();

    hasher.update(content.as_bytes());

    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);

    Metadata {
        name,
        hash,
        prev: vec![],
    }
}

/// Write the metadata and content to a temporary file and renames that file to
/// place it in its permanent location. This prevents partial notes from being
/// written.
fn write_note(metadata: Metadata, content: String) {
    // create the nexus data directory
    let mut data_dir = dirs::data_local_dir().expect("failed to find data directory");

    data_dir.push("nexus");
    fs::create_dir_all(&data_dir).expect("failed to create data directory");

    // create a directory for temporary files
    let mut data_dir_temp = data_dir.clone();

    data_dir_temp.push("tmp");
    fs::create_dir_all(&data_dir_temp).expect("failed to create temp directory");

    // create the path to the new file
    data_dir.push(format!("{}.md", uuid::Uuid::new_v4()));

    let mut temp_file =
        tempfile::NamedTempFile::new_in(data_dir_temp).expect("failed to create temporary file");
    let metadata_str = serde_yaml::to_string(&metadata).expect("failed to serialize metadata");

    // write metadata
    temp_file
        .write_all(format!("---\n{}---\n", metadata_str).as_bytes())
        .expect("failed to write metadata to temporary file");

    // write content
    temp_file
        .write_all(content.as_bytes())
        .expect("failed to write content to temporary file");

    // atomically rename to ensure the entire file is written
    fs::rename(temp_file.path(), data_dir).expect("failed to create permanent file");
}

/// Creates a new note.
fn create_note(name: String) {
    let user_note = create_user_note();
    let content = get_user_content(user_note);
    let metadata = create_metadata(name, &content);

    write_note(metadata, content);
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Create { name } => create_note(name),
    };
}
