use dialoguer::{MultiSelect, theme::ColorfulTheme};

use super::branches::Branch;
pub fn selection(branches: Vec<Branch>) {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which branches do you want to delete?")
        .items(branches.iter().filter(|branch| branch.is_removable()))
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        dbg!(selections);
    }
}
