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
    env,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    process::Command,
};

use tempfile::NamedTempFile;

use crate::{
    errors::NexusResult,
    models::{metadata::Metadata, note::Note},
    utils::write_note,
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

    // create a temporary file and write the note's contents to it
    let note = Note::new(metadata, content);

    write_note(&data_dir, &note, format!("{}.md", uuid::Uuid::new_v4()))?;

    Ok(())
}
