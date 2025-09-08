use std::{
    ffi::OsString,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::HEAD;
use crate::branches::def::Branch;

pub fn get_head(git_dir: &PathBuf) -> OsString {
    let head_file_path = Path::new(&git_dir).join(HEAD);
    let content = read_to_string(head_file_path).expect("Should have been able to read the file");
    let head_path = PathBuf::from(
        content
            .split(" ")
            .last()
            .expect("Failes to get branch")
            .replace("\n", ""),
    );
    let head = head_path.file_name().expect("Failed");
    OsString::from(head)
}

pub fn filter_head(branches: Vec<Branch>) -> Option<Vec<Branch>> {
    let branches = branches
        .into_iter()
        .filter(|branch| branch.is_removable())
        .collect::<Vec<Branch>>();
    match branches.len() {
        0 => None,
        _ => Some(branches),
    }
}
