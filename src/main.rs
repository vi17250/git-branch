use std::{env, fs::read_dir, path::PathBuf};

mod branches;
use branches::utils::{delete_branches, get_branches};
mod commits;
mod dialog;

const REFS_DIR: &str = "refs/heads";
const LOGS_DIR: &str = "logs/refs/heads";
const HEAD: &str = "HEAD";

fn main() -> Result<(), &'static str> {
    let mut current_dir = env::current_dir().expect("Unable to find current directory");

    loop {
        if it_includes_git(&current_dir) {
            current_dir = current_dir.join(".git");
            break;
        } else {
            if let false = current_dir.pop() {
                return Err("This is not a git project");
            }
        }
    }

    let branches = get_branches(current_dir);
    if let None = branches {
        println!(
            "📭 No branches where found\n💡You can create branches using `git branch branch_name`"
        );
        return Ok(());
    }
    let branches_to_delete = dialog::selection(branches.expect("This should not happen"));
    let number_of_deleted_branches = delete_branches(branches_to_delete);
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
