//! Common errors.
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Common error for highlight converter.
#[derive(Debug)]
pub enum HighlightError {
    /// Any error that does not fall in other categories.
    General(String),
    /// Error related to the IO.
    IOError(String, std::io::Error),
    /// Broken format of the input highlights.
    InvalidFormat(String, std::io::Error),
}

impl HighlightError {
    /// Convenience constructor for the IO error.
    pub fn io(message: impl Into<String>, io_error: std::io::Error) -> Self {
        HighlightError::IOError(message.into(), io_error)
    }

    /// Convenience constructor for the format error.
    pub fn format(message: impl Into<String>, io_error: std::io::Error) -> Self {
        HighlightError::InvalidFormat(message.into(), io_error)
    }
}

impl Display for HighlightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HighlightError::General(message) => f.write_str(message),
            HighlightError::IOError(message, err) => {
                f.write_fmt(format_args!("{}\n\t{}", message, err))
            }
            HighlightError::InvalidFormat(message, err) => {
                f.write_fmt(format_args!("{}\n\t{}", message, err))
            }
        }
    }
}

impl Error for HighlightError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HighlightError::IOError(_, err) => Some(err),
            HighlightError::InvalidFormat(_, err) => Some(err),
            HighlightError::General(_) => None,
        }
    }
}

impl Default for HighlightError {
    fn default() -> Self {
        HighlightError::General("unknown error".to_owned())
    }
}
