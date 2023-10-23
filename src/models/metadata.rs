use serde::{Deserialize, Serialize};

/// The metadata associated with a note.
#[derive(Serialize, Deserialize)]
pub struct Metadata {
    /// The user-specified name for the note.
    pub name: String,
    /// The hash of the contents of the note.
    pub hash: String,
    /// A list of branch names and pointers to previous notes.
    pub prev: Vec<(String, String)>,
}
