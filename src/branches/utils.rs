use crate::Result;
use crate::branches::def::Branch;
use crate::branches::head::get_head;
use crate::commits::def::Commit;
use crate::{LOGS_DIR, REFS_DIR};
use std::{
    fs::{read_dir, read_to_string, remove_file},
    path::{Path, PathBuf},
};

use super::head::get_origin_head;

pub fn get_branches(git_dir: &PathBuf) -> Result<Vec<Branch>> {
    let refs_dir = Path::new(&git_dir).join(REFS_DIR);
    let logs_dir = Path::new(&git_dir).join(LOGS_DIR);
    let head = get_head(&git_dir);
    let origin_head = get_origin_head(&git_dir);

    let branches = read_dir(&refs_dir)?
        .map(|entry| match entry {
            Ok(entry) => {
                let time = entry
                    .metadata()
                    .expect("Failed to parse metadata")
                    .modified()
                    .expect("Failed to parse system time");

                let branch_name = entry.path().file_name().expect("WTF").to_os_string();
                let commit_hash = read_to_string(entry.path()).expect("Failed to read commit hash");
                return Branch::new(
                    branch_name.clone(),
                    PathBuf::from(&refs_dir).join(&branch_name),
                    PathBuf::from(&logs_dir).join(&branch_name),
                    *branch_name == head,
                    *branch_name == origin_head,
                    time,
                    Commit::new(commit_hash),
                );
            }
            Err(_) => panic!("Failed to parse entry"),
        })
        .filter(|branch| branch.is_removable())
        .collect::<Vec<Branch>>();
    Ok(branches)
}

pub fn delete_branches(branches: Vec<Branch>) -> Result<usize> {
    let mut count: usize = 0;
    for branch in branches {
        remove_file(branch.get_paths().0).expect("Failed to delete branch");
        remove_file(branch.get_paths().1).expect("Failed to delete branch");
        count += 1;
    }
    Ok(count)
}
