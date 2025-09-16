use dialoguer::{MultiSelect, theme::ColorfulTheme};

use crate::branches::def::Branch;
pub fn selection(branches: Vec<Branch>) -> Vec<Branch> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which branches do you want to delete?")
        .items(branches.iter())
        .interact()
        .expect("Failed to read branches to remove");

    filter_branches(branches, selection)
}

fn filter_branches(branches: Vec<Branch>, selection: Vec<usize>) -> Vec<Branch> {
    selection
        .iter()
        .map(|&index| branches.get(index).unwrap().clone())
        .collect::<Vec<Branch>>()
}

#[allow(warnings)]
mod test {
    use std::{ffi::OsString, path::PathBuf, time::SystemTime};

    use crate::branches::def::Branch;
    use crate::commits::def::Commit;
    use crate::dialog::filter_branches;

    #[test]
    fn it_returns_empty_collection() {
        let b1 = Branch::new(
            OsString::from("b1"),
            PathBuf::new(),
            PathBuf::new(),
            false,
            false,
            SystemTime::now(),
            Commit::new("hash1".to_string()),
        );
        let b2 = Branch::new(
            OsString::from("b2"),
            PathBuf::new(),
            PathBuf::new(),
            false,
            false,
            SystemTime::now(),
            Commit::new("hash2".to_string()),
        );
        assert_eq!(filter_branches(vec![b1, b2], vec![]), vec![])
    }
    #[test]
    fn it_returns_one_branche() {
        let b1 = Branch::new(
            OsString::from("b1"),
            PathBuf::new(),
            PathBuf::new(),
            false,
            false,
            SystemTime::now(),
            Commit::new("hash1".to_string()),
        );
        let b2 = Branch::new(
            OsString::from("b2"),
            PathBuf::new(),
            PathBuf::new(),
            false,
            false,
            SystemTime::now(),
            Commit::new("hash2".to_string()),
        );
        assert_eq!(
            filter_branches(vec![b1.clone(), b2.clone()], vec![1]),
            vec![b2.clone()]
        )
    }

    #[test]
    fn it_returns_all_branches() {
        let b1 = Branch::new(
            OsString::from("b1"),
            PathBuf::new(),
            PathBuf::new(),
            false,
            false,
            SystemTime::now(),
            Commit::new("hash1".to_string()),
        );
        let b2 = Branch::new(
            OsString::from("b2"),
            PathBuf::new(),
            PathBuf::new(),
            false,
            false,
            SystemTime::now(),
            Commit::new("hash2".to_string()),
        );
        assert_eq!(
            filter_branches(vec![b1.clone(), b2.clone()], vec![0, 1]),
            vec![b1.clone(), b2.clone()]
        )
    }
}
