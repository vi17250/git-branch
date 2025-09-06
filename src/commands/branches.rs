use crate::REFS_DIR;
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Branch {
    name: OsString,
    ref_dir: PathBuf,
    is_head: bool,
}

impl Branch {
    fn new(path: PathBuf) -> Branch {
        let name = path.file_name().expect("Failed to parse file name").into();
        Branch {
            name,
            ref_dir: path,
            is_head: false,
        }
    }

    fn set_is_head(&mut self, head: &OsString) {
        if *head == self.name {
            self.is_head = true
        }
    }
}

pub fn get_branches(git_dir: PathBuf) {
    let refs_dir = Path::new(&git_dir).join(REFS_DIR);
    let head = get_head(&git_dir);

    let mut branches = read_dir(refs_dir)
        .expect("Failes to read dir")
        .map(|entry| {
            let branch_path: PathBuf = entry.expect("Failes to parse branch path").path();
            return Branch::new(branch_path);
        })
        .collect::<Vec<Branch>>();

    branches
        .iter_mut()
        .for_each(|mut branch| branch.set_is_head(&head));

    dbg!(branches);
}

fn get_head(git_dir: &PathBuf) -> OsString {
    let head_file_path = Path::new(&git_dir).join("HEAD");
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
