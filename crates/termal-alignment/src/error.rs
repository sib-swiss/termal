use std::fmt;

#[derive(Debug)]
pub enum AlignmentError {
    Io(std::io::Error),
    Parse { msg: String },
    InvalidFormat { msg: String },
}

impl fmt::Display for AlignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignmentError::Io(e) => write!(f, "I/O error: {e}"),
            AlignmentError::Parse { msg } => write!(f, "Parse error: {msg}"),
            AlignmentError::InvalidFormat { msg } => write!(f, "Invalid format: {msg}"),
        }
    }
}

impl std::error::Error for AlignmentError {}

impl From<std::io::Error> for AlignmentError {
    fn from(e: std::io::Error) -> Self { AlignmentError::Io(e) }
}
