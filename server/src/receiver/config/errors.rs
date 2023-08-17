use std::{error::Error, fmt};

#[derive(Debug)]
pub struct UdpReceiverConfigError {
    pub message: String,
}

impl Error for UdpReceiverConfigError {}

impl fmt::Display for UdpReceiverConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
