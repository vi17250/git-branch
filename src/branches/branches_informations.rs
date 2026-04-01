use anyhow::{Context, Result};
use std::time::SystemTime;
use std::{fs::read_to_string, path::Path};

/// Returns the branches informations
///
/// Each files included in the .git/refs/head is a branch
/// This function iterates over the directory and returns a vector of tuple
/// Files represent data of a branch:
/// - name is the file name
/// - last_update is the last time update of the file
/// - commit_hash is the content of the file
pub fn get_branches_informations(refs_dir: &Path) -> Result<Vec<(String, String, SystemTime)>> {
    let entries = std::fs::read_dir(refs_dir)?;
    let branches = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let file_name = path.file_name()?.to_string_lossy().into_owned();
            let commit_hash = read_to_string(&path)
                .context("Failed to read HEAD file")
                .ok()?;
            let last_update = path.metadata().ok()?.modified().ok()?;
            Some((file_name, commit_hash, last_update))
        })
        .collect::<Vec<(String, String, SystemTime)>>();
    Ok(branches)
}
