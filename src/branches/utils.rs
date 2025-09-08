use crate::branches::def::Branch;
use crate::{HEAD, LOGS_DIR, REFS_DIR};
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

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
    filter_head(branches)
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

fn filter_head(branches: Vec<Branch>) -> Vec<Branch> {
    branches
        .into_iter()
        .filter(|branch| branch.is_removable())
        .collect()
}

#[allow(warnings)]
mod test {

    use crate::branches::def::Branch;
    use crate::branches::utils::filter_head;
    use std::{ffi::OsString, path::PathBuf, time::SystemTime};

    #[test]
    fn it_returns_branches_without_head() {
        let b1 = Branch::new(
            OsString::from("b1"),
            PathBuf::new(),
            PathBuf::new(),
            true,
            SystemTime::now(),
        );
        let b2 = Branch::new(
            OsString::from("b2"),
            PathBuf::new(),
            PathBuf::new(),
            false,
            SystemTime::now(),
        );
        assert_eq!(filter_head(vec![b1.clone(), b2.clone()]), vec![b2]);
    }
}
