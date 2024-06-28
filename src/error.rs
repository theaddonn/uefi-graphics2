use core::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UefiDisplayError {
    UnsupportedFormat,
}

impl Display for UefiDisplayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UefiDisplayError => {
                write!(f, "Unsupported Color Format")
            }
        }
    }
}
