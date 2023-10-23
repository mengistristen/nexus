use thiserror::Error;

pub type NexusResult<T> = Result<T, NexusError>;

#[derive(Error, Debug)]
pub enum NexusError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerializationError(#[from] serde_yaml::Error),
}
