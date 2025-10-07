use crate::Result;
use crate::branches::def::Branch;
use crate::branches::{head::get_head, origin::get_origin};
use crate::commits::def::Commit;
use crate::{LOGS_DIR, REFS_DIR};
use std::{
    fs::{read_to_string, remove_file},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn get_branches(git_dir: &PathBuf) -> Result<Vec<Branch>> {
    let refs_dir = Path::new(&git_dir).join(REFS_DIR);
    let logs_dir = Path::new(&git_dir).join(LOGS_DIR);
    let head = get_head(&git_dir)?;
    let origin = get_origin(&git_dir);

    let branches_name = get_branches_name(&refs_dir);
    let branches = branches_name?
        .iter()
        .map(|branch_name| {
            let path = Path::new(&refs_dir).join(branch_name);
            let time = &path
                .metadata()
                .expect("Failed to parse metadata")
                .modified()
                .expect("Failed to parse system time");
            let commit_hash = read_to_string(&path).expect("Failed to read commit hash");
            let is_origin = match &origin {
                Ok(origin) => **branch_name == **origin,
                Err(_) => false,
            };
            return Branch::new(
                branch_name.clone().into(),
                PathBuf::from(&refs_dir).join(&branch_name),
                PathBuf::from(&logs_dir).join(&branch_name),
                **branch_name == *head,
                is_origin,
                *time,
                Commit::new(commit_hash),
            );
        })
        .collect::<Vec<Branch>>();
    Ok(branches)
}

fn get_branches_name(refs_dir: &PathBuf) -> Result<Vec<String>> {
    let mut names: Vec<String> = vec![];
    let refs_dir_name = refs_dir.to_str().ok_or("Failed to convert dir to str")?;

    for entry in WalkDir::new(refs_dir.to_str().expect("WTF")) {
        let entry = entry?;
        if entry.path().is_file() {
            let path = entry.path().display().to_string();
            let mut name = path.replace(refs_dir_name, "");
            names.push(name.split_off(1));
        }
    }
    Ok(names)
}

pub fn delete_branches(branches: Vec<Branch>) -> Result<usize> {
    let mut count: usize = 0;
    for branch in branches {
        remove_file(branch.get_paths().0)?;
        remove_file(branch.get_paths().1)?;
        count += 1;
    }
    Ok(count)
}
