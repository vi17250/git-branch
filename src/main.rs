use anyhow::{Result, anyhow, Context};
use console::style;
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

mod branches;
use branches::def::Branch;

mod util;

mod file_system;

const COMMIT_EDITMSG: &str = "COMMIT_EDITMSG";
const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";

static GIT_DIR: OnceLock<PathBuf> = OnceLock::new();

fn main() -> Result<()> {
    let git_dir = file_system::git_dir()?;
    GIT_DIR.set(git_dir).map_err(|_|anyhow!("Failed to init GIT_DIR"))?;
    let git_dir = GIT_DIR.get().context("Failed to retrieve .git directory")?;

    let commit_editmsg = Path::new(&git_dir).join(COMMIT_EDITMSG);
    if !commit_editmsg.exists() {
        return Err(anyhow!(format!("⚠️ At least one commit must be created")));
    }

    let mut branches: Vec<Branch> = branches::init()?;

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

    let number_of_deleted_branches = branches::delete(branches_to_delete.0)?;
    println!("{} branches deleted", number_of_deleted_branches);

    Ok(())
}
