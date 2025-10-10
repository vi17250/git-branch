use console::style;
use std::{env, fs::read_dir, path::PathBuf};

mod branches;
use branches::utils::{delete_branches, get_branches};

mod commits;
mod dialog;
mod util;

mod error;
pub use crate::error::{Error, Result};

const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";
const ORIGIN_DIR: &str = "refs/remotes/origin/HEAD";

fn main() -> Result<()> {
    let mut current_dir = env::current_dir()?;

    loop {
        if it_includes_git(&current_dir)? {
            current_dir = current_dir.join(".git");
            break;
        }
        if !current_dir.pop() {
            return Err(
                "This is not a git repository\n💡You can create it running `git init`".into(),
            );
        }
    }

    let mut branches = get_branches(&current_dir)?;

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

    let head_branch_index = branches
        .iter()
        .position(|branch| branch.is_head())
        .ok_or("It appears that you have not yet created any commits.")?;

    let head_branch = branches.remove(head_branch_index);

    let head = format!(
        "{} {} {}",
        style("HEAD").color256(6).bold(),
        style("->").color256(202),
        head_branch
    );

    let intro = style("Which branches do you want to delete?").bold();
    println!("{origin}\n{head}\n\n{intro}");

    let branches_to_delete = dialog::selection(branches)?;
    let number_of_deleted_branches = delete_branches(&current_dir, branches_to_delete)?;
    println!("{} branches deleted", number_of_deleted_branches);

    Ok(())
}

fn it_includes_git(dir: &PathBuf) -> Result<bool> {
    let entries = read_dir(dir)?;
    let result = entries
        .map(|entry| entry.expect("Failed to parse Dir name").file_name())
        .find(|value| value == ".git");
    match result {
        None => Ok(false),
        _ => Ok(true),
    }
}
