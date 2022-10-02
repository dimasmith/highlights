use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod highlights;
pub mod input;
pub mod render;

#[derive(Debug)]
pub enum HighlightError {
    Unknown(String),
    IOError(std::io::Error),
}

impl Display for HighlightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HighlightError::Unknown(message) => f.write_str(message),
            HighlightError::IOError(err) => f.write_fmt(format_args!("{}", err)),
        }
    }
}

impl Error for HighlightError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HighlightError::IOError(err) => Some(err),
            HighlightError::Unknown(_) => None,
        }
    }
}

impl Default for HighlightError {
    fn default() -> Self {
        HighlightError::Unknown("unknown error".to_owned())
    }
}

impl From<std::io::Error> for HighlightError {
    fn from(io_error: std::io::Error) -> Self {
        HighlightError::IOError(io_error)
    }
}
