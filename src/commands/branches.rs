use crate::REFS_DIR;
use std::{
    ffi::OsString,
    fs::read_dir,
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
}

pub fn get_branches(git_dir: PathBuf) {
    let refs_dir = Path::new(&git_dir).join(REFS_DIR);

    let branches = read_dir(refs_dir)
        .expect("Failes to read dir")
        .map(|entry| {
            let branch_path: PathBuf = entry.expect("Failes to parse branch path").path();
            return Branch::new(branch_path);
        })
        .collect::<Vec<Branch>>();
    dbg!(branches);
}

