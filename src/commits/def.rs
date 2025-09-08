#[derive(Debug, Clone, PartialEq)]
pub struct Commit(String);

impl Commit {
    pub fn new(hash: String) -> Commit {
        Commit(hash.replace("\n", ""))
    }
}
