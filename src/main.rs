use console::style;
use std::path::Path;
use valinta::select;

mod branches;
use branches::def::Branch;
use branches::utils::{delete_branches, get_branches};

mod util;

mod file_system;

mod error;
pub use crate::error::{Error, Result};

const COMMIT_EDITMSG: &str = "COMMIT_EDITMSG";
const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";

fn main() -> Result<()> {

    let git_dir = file_system::git_dir()?;

    let commit_exist: bool = Path::new(&git_dir).join(COMMIT_EDITMSG).exists();
    if !commit_exist {
        return Err("It appears that you have not yet created any commits".into());
    }

    let mut branches: Vec<Branch> = get_branches(&git_dir)?;

    let head_branch = branches
        .iter()
        .position(|branch| branch.is_head())
        .map(|index| branches.remove(index));

    let head_message: String = match head_branch {
        Some(branch) => format!(
            "{} {} {}",
            style("HEAD").color256(6).bold(),
            style("->").color256(202),
            branch
        ),
        None => String::new(),
    };

    if branches.is_empty() {
        return Err("No branches to delete".into());
    }

    let intro = style("Which branches do you want to delete?").bold();
    println!("{head_message}\n\n{intro}");

    let branches_to_delete = select(&branches)?.0;

    let number_of_deleted_branches = delete_branches(&git_dir, branches_to_delete)?;
    println!("{} branches deleted", number_of_deleted_branches);

    Ok(())
}
