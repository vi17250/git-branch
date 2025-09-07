use dialoguer::{MultiSelect, theme::ColorfulTheme};

use super::branches::Branch;
pub fn selection(branches: Vec<Branch>) -> Vec<Branch> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which branches do you want to delete?")
        .items(branches.iter().filter(|branch| branch.is_removable()))
        .interact()
        .expect("Failed to read branches to remove");

    selection
        .iter()
        .map(|&index| branches.get(index).unwrap().clone())
        .collect::<Vec<Branch>>()
}
