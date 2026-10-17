use std::fmt;

/// Failed to serialize taba script input.
#[derive(Debug)]
pub struct Error(pub(crate) serde_json::Error);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed taba script input serialization: {}", self.0)
    }
}

impl std::error::Error for Error {}
