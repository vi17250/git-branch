use console::style;
use std::path::Path;
use valinta::select;

mod branches;
use branches::def::Branch;
use branches::utils::{delete_branches, get_branches};

mod dialog;
use dialog::confirm::confirm;
mod util;

mod file_system;

mod error;
pub use crate::error::{Error, Result};

const COMMIT_EDITMSG: &str = "COMMIT_EDITMSG";
const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";
const ORIGIN_DIR: &str = "refs/remotes/origin/HEAD";

fn main() -> Result<()> {

    let git_dir = file_system::git_dir()?;

    let commit_exist: bool = Path::new(&git_dir).join(COMMIT_EDITMSG).exists();
    if !commit_exist {
        return Err("It appears that you have not yet created any commits".into());
    }

    let mut branches: Vec<Branch> = get_branches(&git_dir)?;

    let origin_branch = branches.iter().find(|branch| branch.is_origin());

    let origin = match origin_branch {
        Some(origin_branch) => format!(
            "{} {} {}",
            style("origin").red().bold(),
            style("/").red(),
            origin_branch
        ),
        None => "origin is empty on this repository".to_string(),
    };

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
    println!("{origin}\n{head_message}\n\n{intro}");

    let mut branches_to_delete = select(&branches)?.0;

    confirm(&mut branches_to_delete);
    let number_of_deleted_branches = delete_branches(&git_dir, branches_to_delete)?;
    println!("{} branches deleted", number_of_deleted_branches);

    Ok(())
}
