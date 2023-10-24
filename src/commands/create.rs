//! This module is used by the create subcommand to create new notes.
//!
//! Similar to what git will do when you create a commit, this will
//! spawn an editor to allow the user to write the contents of their note
//! before saving it off.
//!
//! Example:
//!
//! ```
//! nexus-cli create "My Note"
//! ```
use std::{
    env, fs,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    process::Command,
};

use tempfile::NamedTempFile;

use crate::{
    errors::NexusResult,
    models::{metadata::Metadata, note::Note},
};

/// Creates a temporary file and spawns an editor to allow the user to write
/// their note.
///
/// # Returns
///
/// Returns The temporary file.
fn create_user_note() -> NexusResult<NamedTempFile> {
    // have the user write to a temporary file
    let file = NamedTempFile::new()?;

    // use whichever editor the user prefers, default to vim
    let editor = env::var("EDITOR").unwrap_or("vim".to_owned());

    // spawn a process to allow the user to write their note;
    // we'll add metadata later
    Command::new(editor).arg(file.path().as_os_str()).status()?;

    Ok(file)
}

/// Reads the user's note from a temporary file.
///
/// # Parameters
///
/// - `file`: the temporary file.
///
/// # Returns
///
/// Returns the contents of the temporary file as a String.
fn get_user_content(mut file: NamedTempFile) -> NexusResult<String> {
    file.seek(SeekFrom::Start(0))?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

/// Writes the metadata and content to a temporary file and renames that file to
/// place it in its permanent location. This prevents partial notes from being
/// written.
///
/// # Parameters
///
/// - `data_dir`: The directory in which to store the note.
/// - `metadata`: The note's metadata.
/// - `content`: The note's content.
fn write_note(data_dir: PathBuf, metadata: Metadata, content: String) -> NexusResult<()> {
    // create a directory for temporary files
    let mut data_dir_temp = data_dir.clone();

    data_dir_temp.push("tmp");
    fs::create_dir_all(&data_dir_temp)?;

    // create the path to the new file
    let mut file_path = data_dir.clone();

    file_path.push(format!("{}.md", uuid::Uuid::new_v4()));

    // create a temporary file and write the note's contents to it
    let mut temp_file = tempfile::NamedTempFile::new_in(data_dir_temp)?;
    let note = Note::new(metadata, content);

    note.write(&mut temp_file)?;

    // atomically rename to ensure the entire file is written
    fs::rename(temp_file.path(), file_path)?;

    Ok(())
}

/// Creates a new note.
///
/// # Parameters
///
/// - `data_dir`: The directory in which to store the note.
/// - `name`: The name of the note.
pub fn create_note(data_dir: PathBuf, name: String) -> NexusResult<()> {
    let user_note = create_user_note()?;
    let content = get_user_content(user_note)?;

    let metadata = Metadata::new(name, &content);

    write_note(data_dir, metadata, content)?;

    Ok(())
}
