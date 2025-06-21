use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum UefiDisplayError {
    #[error("Unsupported Color Format")]
    UnsupportedFormat,
    #[error("Invalid Resolution")]   
    InvalidResolution,
    #[error("Out of Bounds")]
    OutOfBounds,
}
