use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ServerDataError {
    pub message: String,
}

impl ServerDataError {
    pub fn new(message: String) -> Box<Self> {
        Box::new(Self { message })
    }
}

impl Error for ServerDataError {}

impl fmt::Display for ServerDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ServerDataError: {}", self.message)
    }
}
