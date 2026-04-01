use anyhow::{Context, Result};
use std::fs::{remove_dir, remove_file};
use std::path::Path;

use crate::branches::def::Branch;
use crate::{GIT_DIR, LOGS_DIR, REFS_DIR};

///Delete branches recursively
///
///This function takes a vector of branches to delete and return
///the number of branches deleted.
///It invoke another function called `remove()` which is responsible for the deletion of a branch
pub fn delete(branches_to_delete: Vec<Branch>) -> Result<usize> {
    let mut count: usize = 0;
    for branch in branches_to_delete {
        remove(branch.get_name())?;
        count += 1;
    }
    Ok(count)
}

///Delete branch
///
///This function is responsible for deleting the branch given in parameter
///It removes some files from
///-REFS_DIR directory (.git/refs/heads)
///-LOGS_DIR directory (.git/logs/refs/headds)
///
///It also deletes recursively le folder of a branch
///e.g. if the branch name is _"feature/add/user"_ it will also remove /feature/add directory (if empty)
fn remove(name: String) -> Result<()> {
    let git_dir = GIT_DIR.get().context("Failed to retrieve .git directory")?;
    let refs_dir = Path::new(&git_dir).join(REFS_DIR);
    let logs_dir = Path::new(&git_dir).join(LOGS_DIR);

    let path = Path::new(&refs_dir).join(&name);
    remove_file(path)?;
    let path = Path::new(&logs_dir).join(&name);
    remove_file(path)?;

    let mut paths = name.split("/").collect::<Vec<&str>>();

    while paths.len() > 1 {
        paths.pop();
        let to_del = paths.join("/");
        let path = Path::new(&refs_dir).join(&to_del);
        remove_dir(path).ok();
        let path = Path::new(&logs_dir).join(&to_del);
        remove_dir(path).ok();
    }

    Ok(())
}
