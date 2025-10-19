use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    hash: String,
    timestamp: SystemTime,
    message: String,
}

impl Commit {
    pub fn new(values: (String, u64, String)) -> Commit {
        let hash = values.0.trim().to_string();
        let timestamp = UNIX_EPOCH + Duration::from_secs(values.1);
        let message = values.2;
        Commit {
            hash,
            timestamp,
            message,
        }
    }
}

impl Commit {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
}

impl Commit {
    pub fn get_timestamp(&self) -> SystemTime {
        self.timestamp
    }
}

impl Commit {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
