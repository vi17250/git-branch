use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result},
    time::{Duration, SystemTime},
};

use console::style;
use truncrate::TruncateToBoundary;

use crate::util::parse_time;

const NAME_MAX_LENGTH: usize = 10;

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

    fn display(&self) -> String {
        let name = self.get_name();
        let name = if name.len() > NAME_MAX_LENGTH {
            Cow::Owned(format!("{}...", &name[..NAME_MAX_LENGTH - 3]))
        } else {
            Cow::Borrowed(name.as_str())
        };

        let diff = self
            .last_update
            .elapsed()
            .unwrap_or(Duration::ZERO)
            .as_secs();

        let last_update = parse_time(&diff);
        let commit_hash = self.commit_hash.truncate_to_boundary(7).to_string();
        format!("{:NAME_MAX_LENGTH$.NAME_MAX_LENGTH$}| {} | {}", style(name).bold(), commit_hash, last_update)
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let informations = self.display();

        write!(f, "{}", informations)
    }
}
