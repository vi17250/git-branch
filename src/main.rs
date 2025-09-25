use console::style;
use std::{env, fs::read_dir, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod branches;
use branches::utils::{delete_branches, get_branches};
mod commits;
mod dialog;
mod util;

const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";
const ORIGIN_DIR: &str = "refs/remotes/origin/HEAD";

fn main() -> Result<()> {
    let mut current_dir = env::current_dir()?;

    loop {
        if it_includes_git(&current_dir) {
            current_dir = current_dir.join(".git");
            break;
        } else {
            if let false = current_dir.pop() {
                return Err(
                    "This is not a git repository\n💡You can create it running `git init`".into(),
                );
            }
        }
    }

    let mut branches = get_branches(&current_dir).unwrap();

    let origin_branch = branches
        .iter()
        .find(|branch| branch.is_origin())
        .expect("Failed to find origin");

    let origin = format!(
        "{} {} {}",
        style("origin").red().bold(),
        style("/").red(),
        origin_branch
    );

    let head_branch_index = branches
        .iter()
        .position(|branch| branch.is_head())
        .expect("Failed to find head branch");

    let head_branch = branches.remove(head_branch_index);

    let head = format!(
        "{} {} {}",
        style("HEAD").color256(6).bold(),
        style("->").color256(202),
        head_branch
    );

    let intro = style("Which branches do you want to delete?").bold();
    println!("\n{head}\n{origin}\n\n{intro}");

    let branches_to_delete = dialog::selection(branches);
    let number_of_deleted_branches = delete_branches(branches_to_delete)?;
    println!("{} branches deleted", number_of_deleted_branches);

    Ok(())
}

fn it_includes_git(dir: &PathBuf) -> bool {
    let entries = read_dir(dir).expect("Failes to read dir");
    let result = entries
        .map(|entry| entry.expect("Failed to parse Dir name").file_name())
        .find(|value| value == ".git");
    match result {
        None => false,
        _ => true,
    }
}
