use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

mod branches;
use branches::utils::get_branches;
mod dialog;

const ROOT_DIR: &str = ".";
const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";

fn main() {
    let mut git_dir: PathBuf = PathBuf::from(ROOT_DIR);
    loop {
        if it_includes_git(git_dir.clone()) {
            git_dir.push(".git/");
            break;
        } else {
            git_dir = Path::new("../").join(git_dir);
        }
    }
    let branches = get_branches(git_dir);
    let to_del = dialog::selection(branches);
    dbg!(to_del);
}

fn it_includes_git(dir: PathBuf) -> bool {
    let entries = read_dir(dir).expect("Failes to read dir");
    let result = entries
        .map(|entry| entry.expect("Failed to parse Dir name").file_name())
        .find(|value| value == ".git");
    match result {
        None => false,
        _ => true,
    }
}
