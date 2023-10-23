use serde::{Deserialize, Serialize};
use sha1::Digest;

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

impl Metadata {
    pub fn new(name: String, content: &String) -> Self {
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
}
