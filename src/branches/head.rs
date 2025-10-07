use std::{
    ffi::OsString,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::HEAD;
use crate::Result;

pub fn get_head(git_dir: &PathBuf) -> Result<OsString> {
    let head_file_path = Path::new(&git_dir).join(HEAD);
    let content = read_to_string(head_file_path)?;
    let head_path = PathBuf::from(
        content
            .split(" ")
            .last()
            .ok_or("Failed to get head branch")?
            .replace("\n", ""),
    );
    let head = head_path.file_name().ok_or("Failed to read head")?;
    Ok(OsString::from(head))
}
