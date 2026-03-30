use crate::error::Result;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;

/// Determines whether the current directory is a _Git_ directory
/// 
/// This function traverses the folder structure until it finds a directory named _.git_
/// It's only possible action is to move up to it's parent.
/// 
/// # Example
/// ```rust
///     let current_git_dir = current_dir()?;
/// ```
pub fn git_dir() -> Result<PathBuf> {
    let mut current_dir = env::current_dir()?;

    loop {
        if includes_git(&current_dir)? {
            current_dir = current_dir.join(".git");
            break;
        }
        if !current_dir.pop() {
            return Err(
                "This is not a git repository -> 💡 You can create it running `git init`".into(),
            );
        }
    }
    Ok(current_dir)
}

/// Find _.git_ directory in the directory in parameter.
/// 
/// Reads the children directory until it finds _.git_ directory.
/// It panics if a problem arises
/// 
/// # Argument
/// *`dir` the directory to explore
/// 
/// # Return
/// * `Ok(true)` if _.git_ directory exists
/// * `Ok(false)` if _.git_ not exists
/// * `Err()` if a problem occurs
/// 
/// #Example
/// ```ignore
/// let path = Path::from("a/path/to/explore");
/// match includes_git(&path){
///     Ok(true)=> println!(".git exists"),
///     Ok(false)=> println!(".git not exists"),
///     Err(e)=> eprintln!("An error occurs {:?}", e),
/// }
/// ```
fn includes_git(dir: &PathBuf) -> Result<bool> {
    let entries = read_dir(dir)?;
    
    let result = entries
        .map(|entry| entry.expect("Failed to parse dirname").file_name())
        .find(|value| value == ".git");
    Ok(result.is_some())
}
