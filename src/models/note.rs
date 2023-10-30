//! This module contains the definition of a note.
use std::{fs, io::Write, path::PathBuf};

use crate::errors::{NexusResult, NoteError};

use super::metadata::Metadata;

/// A note containing metadata and string contents.
pub struct Note {
    /// The note's metadata.
    pub metadata: Metadata,
    /// The contents of the note.
    contents: String,
}

impl Note {
    /// Creates a new note.
    ///
    /// # Parameters
    ///
    /// - `metadata`: The metadata associated with the note.
    /// - `contents`: The contents of the note.
    pub fn new(metadata: Metadata, contents: String) -> Self {
        Self { metadata, contents }
    }

    /// Converts a note into a `String`.
    pub fn to_string(&self) -> NexusResult<String> {
        let frontmatter = serde_yaml::to_string(&self.metadata)?;
        let combined = format!("---\n{}\n---\n{}", frontmatter, self.contents);

        Ok(combined)
    }

    /// Constructs a note from a `String`.
    ///
    /// # Parameters
    ///
    /// - `source`: The `String` from which to construct the note.
    ///
    /// # Returns
    ///
    /// The resulting `Note`.
    pub fn from_string(source: String) -> NexusResult<Self> {
        let parts: Vec<&str> = source.split("---\n").collect();

        if parts.len() != 3 {
            return Err(NoteError::Deserialization)?;
        }

        let metadata = serde_yaml::from_str(parts[1])?;

        Ok(Note {
            metadata,
            contents: parts[2].to_owned(),
        })
    }

    /// Reads the contents of a file and constructs a new `Note` from them.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the file to read.
    ///
    /// # Returns
    ///
    /// The resulting `Note`.
    pub fn from_file(path: PathBuf) -> NexusResult<Self> {
        let contents = fs::read_to_string(path)?;

        Self::from_string(contents)
    }

    /// Writes a note to the given writer.
    ///
    /// # Parameters
    ///
    /// - `writer`: The writer to write to.
    pub fn write<W: Write>(&self, writer: &mut W) -> NexusResult<()> {
        let data = self.to_string()?;

        write!(writer, "{}", data)?;

        Ok(())
    }
}
