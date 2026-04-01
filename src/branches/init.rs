use anyhow::{Context, Result};
use std::path::Path;

use crate::branches::def::Branch;
use crate::branches::head::Head;
use crate::{GIT_DIR, REFS_DIR};

use crate::branches::get_branches_informations;

/// Returns a collection of branches
///
/// Each fields (excepted `is_head`) can be determined based on the data
/// contained in the file in the folder `.git/refs/heads`:
/// It's easy to determinate if a branch is the current branch:
/// the current branch is mentionned in the .git/HEAD file
pub fn init() -> Result<Vec<Branch>> {
    let git_dir = GIT_DIR.get().context("Failed to retrieve .git directory")?;
    let refs_dir = Path::new(&git_dir).join(REFS_DIR);
    let head = Head::init()?.get_name();

    let branches_informations = get_branches_informations(&refs_dir)?;
    let branches = branches_informations
        .iter()
        .map(|branch| {
            let is_head = head.contains(&branch.0);
            let name = branch.0.clone();
            let commit_hash = branch.1.clone();
            let lasf_update = branch.2;
            Ok(Branch::new(name, is_head, lasf_update, commit_hash))
        })
        .flat_map(|branch: Result<Branch>| branch.ok())
        .collect::<Vec<Branch>>();
    Ok(branches)
}
