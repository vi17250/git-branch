use std::{
    ffi::OsString,
    fmt::{Display, Formatter, Result},
    time::SystemTime,
};

use truncrate::TruncateToBoundary;

use crate::util::parse_time;

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    name: OsString,
    is_head: bool,
    is_origin: bool,
    last_update: SystemTime,
    commit_hash: String,
}

impl Branch {
    pub fn new(
        name: OsString,
        is_head: bool,
        is_origin: bool,
        last_update: SystemTime,
        commit_hash: String,
    ) -> Branch {
        Branch {
            name,
            is_head,
            is_origin,
            last_update,
            commit_hash,
        }
    }

    pub fn is_head(&self) -> bool {
        self.is_head
    }

    pub fn is_origin(&self) -> bool {
        self.is_origin
    }

    pub fn get_name(&self) -> OsString {
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
            name.to_str().unwrap(),
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
