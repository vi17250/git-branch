use std::{
    ffi::OsString,
    fmt::{Display, Formatter, Result},
    path::PathBuf,
    time::SystemTime,
};

use crate::commits::def::Commit;

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    name: OsString,
    refs_dir: PathBuf,
    logs_dir: PathBuf,
    is_head: bool,
    is_origin_head: bool,
    last_update: SystemTime,
    commit: Commit,
}

impl Branch {
    pub fn new(
        name: OsString,
        refs_dir: PathBuf,
        logs_dir: PathBuf,
        is_head: bool,
        is_origin_head: bool,
        last_update: SystemTime,
        commit: Commit,
    ) -> Branch {
        Branch {
            name,
            refs_dir,
            logs_dir,
            is_head,
            is_origin_head,
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

        write!(f, "{}\t", name,)
    }
}
