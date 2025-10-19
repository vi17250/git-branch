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
                .truncate_to_boundary(10),
        )
        .bold();

        if self.is_origin {
            name = name.red();
        };

        let (last_update, message) = match &self.commit {
            Some(commit) => {
                let diff = commit
                    .get_timestamp()
                    .elapsed()
                    .expect("Failes to parse last commit update")
                    .as_secs();
                (parse_time(&diff), commit.get_message())
            }
            None => (String::new(), String::new()),
        };

        write!(
            f,
            "{0: <11} {1: <3} {2: <10}",
            name,
            style(last_update).color256(241).italic(),
            style(message.truncate_to_boundary(30)).color256(202),
        )
    }
}
