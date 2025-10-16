use std::fmt::{Display, Formatter, Result};
use truncrate::TruncateToBoundary;

#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    hash: String,
    timestamp: String,
    message: String,
}

impl Commit {
    pub fn new(values: (String, String, String)) -> Commit {
        let hash = values.0.trim().to_string();
        let timestamp = values.1;
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

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let drained = &self.message.truncate_to_boundary(30);
        write!(f, "{drained}")
    }
}
