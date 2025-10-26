use std::{
    ffi::OsString,
    fmt::{Display, Formatter, Result},
    time::SystemTime,
};

use console::style;
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
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut name = style(
            self.name
                .to_str()
                .expect("Failed to parse OsString to String")
                .truncate_to_boundary(10),
        )
        .bold();

        let diff = self
            .last_update
            .elapsed()
            .expect("Failed to parse last update")
            .as_secs();

        let last_update = parse_time(&diff);

        if self.is_origin {
            name = name.red();
        };

        write!(
            f,
            "{0: <11} {1: <3} {2: <10}",
            name,
            style(last_update).color256(241).italic(),
            style(self.commit_hash.truncate_to_boundary(7)).color256(202),
        )
    }
}
