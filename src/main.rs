use anyhow::{Result, anyhow};
use console::style;
use std::path::Path;

mod branches;
use branches::def::Branch;

mod util;

mod file_system;

const COMMIT_EDITMSG: &str = "COMMIT_EDITMSG";
const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";

fn main() -> Result<()> {
    let git_dir = file_system::git_dir()?;

    let commit_msg = Path::new(&git_dir).join(COMMIT_EDITMSG);
    if !commit_msg.exists() {
        return Err(anyhow!(format!("⚠️ At least one commit must be created")));
    }

    let mut branches: Vec<Branch> = branches::init(&git_dir)?;

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
        return Err(anyhow!(format!("⚠️ At least one branch must be created")));
    }

    let intro = style("Which branches do you want to delete?").bold();
    println!("{head_message}\n\n{intro}");

    let branches_to_delete = valinta::select(&branches)?;

    let number_of_deleted_branches = branches::delete(&git_dir, branches_to_delete.0)?;
    println!("{} branches deleted", number_of_deleted_branches);

    Ok(())
}
