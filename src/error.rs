use core::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum UefiDisplayError {
    #[error("Unsupported Color Format")]
    UnsupportedFormat,
}
