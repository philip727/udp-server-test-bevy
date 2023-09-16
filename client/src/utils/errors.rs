use std::{error::Error, fmt};

#[derive(Debug)]
pub struct OpenFileError {
    message: String
}

impl OpenFileError {
    pub fn new(m: String) -> Box<Self> {
        Box::new(OpenFileError { message: m })
    }
}

impl Error for OpenFileError {}

impl fmt::Display for OpenFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to open file: {}", self.message)
    }
}
