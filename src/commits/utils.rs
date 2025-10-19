use crate::Result;
use crate::commits::def::Commit;
use std::{fs::read_to_string, path::PathBuf};

use regex::Regex;

pub fn get_commits(branch_path: PathBuf) -> Result<Vec<Commit>> {
    let content = read_to_string(branch_path)?;
    let commits = content
        .lines()
        .filter(|line| line.contains("\tcommit"))
        .map(|line| {
            let values = get_commit_values(line).expect("Failed to parse String to u64");
            Commit::new(values)
        })
        .collect::<Vec<Commit>>();
    Ok(commits)
}

fn get_commit_values(line: &str) -> Result<(String, u64, String)> {
    let regex = Regex::new(
        r"(\w+)\s(?P<hash>\w+)\s(?P<author>.*>)\s(?P<timestamp>\d{10}).*\scommit\s?(\(\w*\))?:\s?(?P<name>.*)",
    ).expect("Failed to parse regex");
    let caps = regex.captures(line).expect("Failed to parse commit data");
    let hash = &caps["hash"];
    let timestamp = &caps["timestamp"];
    let name = &caps["name"];
    Ok((
        hash.to_string(),
        timestamp.parse::<u64>()?,
        name.to_string(),
    ))
}
mod test {
    use super::*;

    #[test]
    fn it_returns_commit_values() -> Result<()> {
        let line = "0dac748bd210848a29be86f56a2b7fe4e32ee669 55d376baa5ebe0d5fd377f008779778623985e82 vi17250 <v.aubinaud@protonmail.com> 1760637303 +0200 commit: feat: 🎸 commit msg";
        assert_eq!(
            get_commit_values(line)?,
            (
                String::from("55d376baa5ebe0d5fd377f008779778623985e82"),
                1760637303,
                String::from("feat: 🎸 commit msg")
            )
        );
        Ok(())
    }
    #[test]
    fn it_returns_ammend_name() -> Result<()> {
        let line = "0dac748bd210848a29be86f56a2b7fe4e32ee669 55d376baa5ebe0d5fd377f008779778623985e82 vi17250 <v.aubinaud@protonmail.com> 1760637303 +0200 commit (amend): feat: 🎸 commit msg";
        assert_eq!(
            get_commit_values(line)?,
            (
                String::from("55d376baa5ebe0d5fd377f008779778623985e82"),
                1760637303,
                String::from("feat: 🎸 commit msg")
            )
        );
        Ok(())
    }
}
