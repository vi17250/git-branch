use crate::branches::def::Branch;
use crate::branches::head::{filter_head, get_head};
use crate::{LOGS_DIR, REFS_DIR};
use std::{
    fs::{read_dir, remove_file},
    path::{Path, PathBuf},
};

pub fn get_branches(git_dir: PathBuf) -> Option<Vec<Branch>> {
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

pub fn delete_branches(branches: Vec<Branch>) -> usize {
    let mut count: usize = 0;
    for branch in branches {
        remove_file(branch.get_paths().0).expect("Failed to delete branch");
        remove_file(branch.get_paths().1).expect("Failed to delete branch");
        count += 1;
    }
    count
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
        assert_eq!(filter_head(vec![b1.clone(), b2.clone()]), Some(vec![b2]));
    }
}
