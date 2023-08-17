use std::{error::Error, fmt};

pub struct UdpClientError {
    pub message: String
}

impl Error for UdpClientError {}

impl fmt::Display for UdpClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
