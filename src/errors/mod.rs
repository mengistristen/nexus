//! This module contains types used for error handling.
use thiserror::Error;

/// A type alias for `Result<T, NexusError>`
pub type NexusResult<T> = Result<T, NexusError>;

/// The global error type for the application.
#[derive(Error, Debug)]
pub enum NexusError {
    /// A wrapper around an IO error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// A wrapper around a serialization error.
    #[error(transparent)]
    SerializationError(#[from] serde_yaml::Error),
    /// A wrapper around an error pertaining to a `Note`.
    #[error(transparent)]
    NoteError(#[from] NoteError),
}

/// Error type specific to notes.
#[derive(Error, Debug)]
pub enum NoteError {
    #[error("Error deserializing note")]
    DeserializationError,
}
