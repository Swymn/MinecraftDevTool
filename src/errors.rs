use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Unable to determine the project generator")]
    UnableToDetermineProjectGenerator,

    #[error("Unable to create file: {0}")]
    FileCreationError(io::Error),

    #[error("Unable to create directory: {0}")]
    DirectoryCreationError(io::Error),
}
