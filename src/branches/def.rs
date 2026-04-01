use std::{
    fmt::{Display, Formatter, Result},
    time::SystemTime,
};

use truncrate::TruncateToBoundary;

use crate::util::parse_time;

/// Representation of a git branch
#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    name: String,
    is_head: bool,
    last_update: SystemTime,
    commit_hash: String,
}

impl Branch {
    pub fn new(
        name: String,
        is_head: bool,
        last_update: SystemTime,
        commit_hash: String,
    ) -> Branch {
        Branch {
            name,
            is_head,
            last_update,
            commit_hash,
        }
    }

    pub fn is_head(&self) -> bool {
        self.is_head
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn display(&self) -> String {
        let name = self.get_name();
        let diff = self
            .last_update
            .elapsed()
            .expect("Failed to parse last update")
            .as_secs();

        let last_update = parse_time(&diff);
        let commit_hash = self.commit_hash.truncate_to_boundary(7);

        format!(
            "{} -> {} | {}",
            name,
            commit_hash,
            last_update
        )
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let output = self.display();

        write!(f, "{}", output)
    }
}
