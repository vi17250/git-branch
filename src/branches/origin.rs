use std::{
    ffi::OsString,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::ORIGIN_DIR;
use crate::Result;

pub fn get_origin(git_dir: &PathBuf) -> Result<OsString> {
    let head_file_path = Path::new(&git_dir).join(ORIGIN_DIR);
    let content = read_to_string(head_file_path)?;
    
    let head_path = PathBuf::from(
        content
            .split(" ")
            .last()
            .ok_or("Failed to get branch")?
            .replace("\n", ""),
    );
    let origin = head_path.file_name().expect("Failed");
    Ok(OsString::from(origin))
}
