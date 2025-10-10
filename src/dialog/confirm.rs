use crate::branches::def::Branch;
use dialoguer::{Confirm, theme::ColorfulTheme};

pub fn confirm(branches_to_delete: &mut Vec<Branch>) {
    let includes_origin = branches_to_delete.iter().any(|branch| branch.is_origin());
    if includes_origin
        && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you really want to remove origin ?")
            .default(false)
            .show_default(true)
            .wait_for_newline(true)
            .interact()
            .unwrap()
    {
        branches_to_delete.retain(|branch| !branch.is_origin());
    }
}
