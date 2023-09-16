use std::{error::Error, fmt};

#[derive(Debug)]
pub struct WorldStoreError {
    pub message: String,
}

impl WorldStoreError {
    pub fn new(message: String) -> Box<Self> {
        Box::new(Self { message })
    }
}

impl Error for WorldStoreError {}

impl fmt::Display for WorldStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorldStoreError: {}", self.message)
    }
}
