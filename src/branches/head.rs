use anyhow::{Context, Result};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::{GIT_DIR, HEAD};

#[derive(Debug)]
pub struct Head {
    branch_name: String,
}

impl Head {
    /// Create a new HEAD struct which includes the name of the current branch.
    ///
    /// It reads the content if the .git/HEAD file and parses it.
    /// The result of the parsing (the filename) is the name of the current branch
    pub fn init() -> Result<Self> {
        let git_dir = GIT_DIR.get().context("Failed to retrieve .git directory")?;
        let head_file_path: PathBuf = Path::new(&git_dir).join(HEAD);
        let content = read_to_string(head_file_path).context("Failed to read `.git/HEAD` file")?;
        let path = Path::new(&content);
        let branch_name = path
            .file_name()
            .context("Failed to determine HEAD branch")?
            .to_string_lossy()
            .into_owned();

        Ok(Head { branch_name })
    }
}

impl Head {
    pub fn get_name(&self) -> String {
        self.branch_name.clone()
    }
}
