use std::{
    ffi::OsString,
    fmt::{Display, Formatter, Result},
    time::SystemTime,
};

use console::style;
use truncrate::TruncateToBoundary;

use crate::commits::def::Commit;
use crate::util::parse_time;

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    name: OsString,
    is_head: bool,
    is_origin: bool,
    last_update: SystemTime,
    commit: Option<Commit>,
}

impl Branch {
    pub fn new(
        name: OsString,
        is_head: bool,
        is_origin: bool,
        last_update: SystemTime,
        commit: Option<Commit>,
    ) -> Branch {
        Branch {
            name,
            is_head,
            is_origin,
            last_update,
            commit,
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
                .truncate_to_boundary(12),
        )
        .bold();

        if self.is_origin {
            name = name.red();
        };

        let diff = self
            .last_update
            .elapsed()
            .expect("Failed tu parse last update")
            .as_secs();

        let last_update = parse_time(&diff);

        let commit = match &self.commit {
            Some(commit) => format!("{commit}"),
            None => String::new(),
        };

        write!(
            f,
            "{0: <12} {1: <7} {2: <10}",
            name,
            style(last_update).color256(241).italic(),
            style(commit).color256(202),
        )
    }
}
