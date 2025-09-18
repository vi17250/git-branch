use std::process;

use crate::branches::def::Branch;
use console::{Key, Term, style};

pub fn selection(branches: Vec<Branch>) -> Vec<Branch> {
    let mut selected: Vec<usize> = vec![];

    let mut highlight: usize = 0;

    display(&branches, highlight, &selected);

    let raw = std::env::args_os().any(|arg| arg == "-r" || arg == "--raw");
    let term = Term::stdout();

    loop {
        let key = if raw {
            term.read_key_raw()
        } else {
            term.read_key()
        }
        .expect("Failed tu parse key");

        match key {
            Key::ArrowUp => decrement(&mut highlight),
            Key::ArrowDown => increment(&mut highlight, branches.len()),
            Key::Char(' ') => toggle_highlight(highlight, &mut selected),
            Key::Enter => break,
            Key::Escape => process::exit(0),
            _ => (),
        };

        let _ = term.move_cursor_up(branches.len() as usize);

        display(&branches, highlight, &selected);
    }

    filter_branches(branches, selected)
}

fn filter_branches(branches: Vec<Branch>, selection: Vec<usize>) -> Vec<Branch> {
    selection
        .iter()
        .map(|&index| branches.get(index).unwrap().clone())
        .collect::<Vec<Branch>>()
}

fn increment(value: &mut usize, max: usize) {
    if *value <= max - 1 {
        *value += 1
    }
}

fn decrement(value: &mut usize) {
    if *value != 0 {
        *value -= 1
    }
}

fn display(values: &Vec<Branch>, highlight: usize, selected: &Vec<usize>) {
    let term = Term::stdout();
    for (index, value) in values.iter().enumerate() {
        let _ = term.clear_line();
        if selected.into_iter().any(|select| *select == index) {
            print!("{} ", style('✔').green().bold());
        } else {
            print!("{} ", style('☐').color256(0).bold());
        }
        if highlight == index {
            println!("{}", style(value).white().on_green())
        } else {
            println!("{value}");
        }
    }
}

fn toggle_highlight(highlight: usize, selected: &mut Vec<usize>) {
    let includes = selected.into_iter().any(|select| *select == highlight);
    if includes {
        selected.retain(|&value| value != highlight);
    } else {
        selected.push(highlight);
    }
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
            SystemTime::now(),
            Commit::new("hash1".to_string()),
        );
        let b2 = Branch::new(
            OsString::from("b2"),
            PathBuf::new(),
            PathBuf::new(),
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
            SystemTime::now(),
            Commit::new("hash1".to_string()),
        );
        let b2 = Branch::new(
            OsString::from("b2"),
            PathBuf::new(),
            PathBuf::new(),
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
            SystemTime::now(),
            Commit::new("hash1".to_string()),
        );
        let b2 = Branch::new(
            OsString::from("b2"),
            PathBuf::new(),
            PathBuf::new(),
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
