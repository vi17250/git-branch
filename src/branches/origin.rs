use std::{
    ffi::OsString,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::ORIGIN_DIR;

pub fn get_origin(git_dir: &PathBuf) -> Result<OsString, ()> {
    let head_file_path = Path::new(&git_dir).join(ORIGIN_DIR);
    let content = match read_to_string(head_file_path) {
        Ok(content) => content,
        Err(_) => return Err(()),
    };
    let head_path = PathBuf::from(
        content
            .split(" ")
            .last()
            .expect("Failed to get branch")
            .replace("\n", ""),
    );
    let origin = head_path.file_name().expect("Failed");
    Ok(OsString::from(origin))
}
