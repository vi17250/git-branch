use crate::{HEAD, LOGS_DIR, REFS_DIR};
use std::{
    ffi::OsString,
    fmt::{Display, Formatter, Result},
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug)]
pub struct Branch {
    name: OsString,
    refs_dir: PathBuf,
    logs_dir: PathBuf,
    is_head: bool,
    last_update: SystemTime,
}

impl Branch {
    fn new(
        name: OsString,
        refs_dir: PathBuf,
        logs_dir: PathBuf,
        is_head: bool,
        last_update: SystemTime,
    ) -> Branch {
        Branch {
            name,
            refs_dir,
            logs_dir,
            is_head,
            last_update,
        }
    }

    pub fn is_removable(&self) -> bool {
        !self.is_head
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = self
            .name
            .to_str()
            .expect("Failed to parse OsString to String");

        write!(f, "{}\t", name,)
    }
}

pub fn get_branches(git_dir: PathBuf) -> Vec<Branch> {
    let refs_dir = Path::new(&git_dir).join(REFS_DIR);
    let logs_dir = Path::new(&git_dir).join(LOGS_DIR);
    let head = get_head(&git_dir);

    let branches = read_dir(&refs_dir)
        .expect("Failes to read dir")
        .map(|entry| match entry {
            Ok(entry) => {
                let time = entry
                    .metadata()
                    .expect("Failed to parse metadata")
                    .modified()
                    .expect("Failed to parse system time");

                let branch_name = entry.path().file_name().expect("WTF").to_os_string();
                return Branch::new(
                    branch_name.clone(),
                    PathBuf::from(&refs_dir).join(&branch_name),
                    PathBuf::from(&logs_dir).join(&branch_name),
                    *branch_name == head,
                    time,
                );
            }
            Err(_) => panic!("Failed to parse entry"),
        })
        .collect::<Vec<Branch>>();
    branches
}

fn get_head(git_dir: &PathBuf) -> OsString {
    let head_file_path = Path::new(&git_dir).join(HEAD);
    let content = read_to_string(head_file_path).expect("Should have been able to read the file");
    let head_path = PathBuf::from(
        content
            .split(" ")
            .last()
            .expect("Failes to get branch")
            .replace("\n", ""),
    );
    let head = head_path.file_name().expect("Failed");
    OsString::from(head)
}
