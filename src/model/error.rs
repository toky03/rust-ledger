use std::error::Error;
use std::fmt::{format, Display, Formatter, Pointer};

pub type Result<T> = std::result::Result<T, AccError>;

#[derive(Debug, Clone)]
pub struct AccError {
    error_message: String,
}

impl Display for AccError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error_message)
    }
}

impl Error for AccError {}

impl AccError {
    pub fn new(message: String) -> Self {
        AccError {
            error_message: message,
        }
    }
}
