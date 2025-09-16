use std::fmt::{Display, Formatter, Result};
#[derive(Debug, Clone, PartialEq)]
pub struct Commit(String);

impl Commit {
    pub fn new(hash: String) -> Commit {
        Commit(hash.replace("\n", ""))
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let drained = &self.0.as_str()[0..7];
        write!(f, "{drained}")
    }
}
