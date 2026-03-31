use anyhow::{Context, Result, anyhow};
use std::env;
use std::path::PathBuf;

/// Determines whether the current directory is a _Git_ directory
///
/// This function traverses the folder structure until it finds a directory named _.git_
/// It's only possible action is to move up to it's parent.
/// If no one _.git_ is found at the top level, it throw an error
///
/// # Example
/// ```rust
///     let current_git_dir = git_dir()?;
/// ```
pub fn git_dir() -> Result<PathBuf> {
    let mut current_dir = env::current_dir().context("Unable to retrieve the current directory")?;

    loop {
        let new_dir = current_dir.join(".git");
        if new_dir.exists() {
            return Ok(new_dir);
        }
        if !current_dir.pop() {
            return Err(anyhow!(format!(
                "This is not a git repository\n💡 You can create one running `git init`"
            )));
        }
    }
}