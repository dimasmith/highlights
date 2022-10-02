//! Common errors.
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum HighlightError {
    General(String),
    IOError(String, std::io::Error),
}

impl HighlightError {
    pub fn new(message: impl Into<String>, io_error: std::io::Error) -> Self {
        HighlightError::IOError(message.into(), io_error)
    }
}

impl Display for HighlightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HighlightError::General(message) => f.write_str(message),
            HighlightError::IOError(message, err) => {
                f.write_fmt(format_args!("{}\n\t{}", message, err))
            }
        }
    }
}

impl Error for HighlightError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HighlightError::IOError(_, err) => Some(err),
            HighlightError::General(_) => None,
        }
    }
}

impl Default for HighlightError {
    fn default() -> Self {
        HighlightError::General("unknown error".to_owned())
    }
}
