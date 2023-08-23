use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ServerDataError {
    pub message: String,
}

impl Error for ServerDataError {}

impl fmt::Display for ServerDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ServerDataError: {}", self.message)
    }
}
