//! This module is used by the link subcommand to link notes together.
//!
//! A single note can be linked to several other notes via a list of prev
//! pointers. Each prev pointer has an associated branch name that is used
//! when consolidating notes. If link is called without a branch name,
//! it will attempt to link via the 'default' branch name.
//!
//! # Example
//!
//! ```
//! nexus-cli link note_one.md note_two.md my_branch
//! ```
use std::path::PathBuf;

use crate::{
    errors::{NexusResult, NoteError},
    models::note::Note,
    utils::{get_note_file_names, write_note},
};

/// Links a note to a previous note.
///
/// # Parameters
///
/// - `data_dir`: The directory to read for notes.
/// - `source`: The beginning of the file name for the source note.
/// - `target`: The beginning of the file name for the target note.
/// - `branch`: The optional branch name.
pub fn link_note(
    data_dir: PathBuf,
    source: String,
    target: String,
    branch: Option<String>,
) -> NexusResult<()> {
    let note_file_names = get_note_file_names(&data_dir)?;

    let source_name = note_file_names
        .iter()
        .find(|&name| name.starts_with(&source))
        .ok_or(NoteError::DoesNotExist(source))?;
    let target_name = note_file_names
        .iter()
        .find(|&name| name.starts_with(&target))
        .ok_or(NoteError::DoesNotExist(target))?;

    let note_path = data_dir.join(source_name);

    let mut note = Note::from_file(note_path)?;
    let branch = if let Some(name) = branch {
        name
    } else {
        "default".to_owned()
    };

    note.metadata.prev.insert(branch, target_name.clone());

    write_note(&data_dir, &note, source_name.clone())?;

    Ok(())
}
