use std::{
    ffi::OsString,
    fmt::{Display, Formatter, Result},
    path::PathBuf,
    time::SystemTime,
};

use console::style;

use crate::commits::def::Commit;
use crate::util::parse_time;

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    name: OsString,
    refs_dir: PathBuf,
    logs_dir: PathBuf,
    is_head: bool,
    last_update: SystemTime,
    commit: Commit,
}

impl Branch {
    pub fn new(
        name: OsString,
        refs_dir: PathBuf,
        logs_dir: PathBuf,
        is_head: bool,
        last_update: SystemTime,
        commit: Commit,
    ) -> Branch {
        Branch {
            name,
            refs_dir,
            logs_dir,
            is_head,
            last_update,
            commit,
        }
    }

    pub fn is_removable(&self) -> bool {
        !self.is_head
    }

    pub fn get_paths(&self) -> (PathBuf, PathBuf) {
        (self.refs_dir.clone(), self.logs_dir.clone())
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = self
            .name
            .to_str()
            .expect("Failed to parse OsString to String");

        let commit_hash = &self.commit;
        let diff = self
            .last_update
            .elapsed()
            .expect("Failed tu parse last update")
            .as_secs();
        let last_update = parse_time(&diff);

        write!(
            f,
            "{0: <12} {1: <7} {2: <10}",
            style(name).bold(),
            style(commit_hash).color256(202),
            style(last_update).color256(241).italic()
        )
    }
}
