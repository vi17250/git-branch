use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result},
    time::{Duration, SystemTime},
};

use truncrate::TruncateToBoundary;

use crate::util::parse_time;

const NAME_MAX_LENGTH: usize = 16;

/// Representation of a git branch
#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    name: String,
    is_head: bool,
    last_update: SystemTime,
    commit_hash: String,
}

impl Branch {
    pub fn new(
        name: String,
        is_head: bool,
        last_update: SystemTime,
        commit_hash: String,
    ) -> Branch {
        Branch {
            name,
            is_head,
            last_update,
            commit_hash,
        }
    }

    pub fn is_head(&self) -> bool {
        self.is_head
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn display(&self) -> String {
        let name = self.get_name();
        let name = if name.len() > NAME_MAX_LENGTH {
            Cow::Owned(format!("{}...", &name[..NAME_MAX_LENGTH - 3]))
        } else {
            Cow::Borrowed(name.as_str())
        };

        let diff = self
            .last_update
            .elapsed()
            .unwrap_or(Duration::ZERO)
            .as_secs();

        let last_update = parse_time(&diff);
        let commit_hash = self.commit_hash.truncate_to_boundary(7).to_string();
        format!(
            "{:NAME_MAX_LENGTH$.NAME_MAX_LENGTH$}| {} | {}",
            name,
            commit_hash,
            last_update
        )
    }
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let informations = self.display();

        write!(f, "{}", informations)
    }
}

#[allow(warnings)]
mod test {
    use std::time::SystemTime;

    use crate::branches::def::Branch;

    #[test]
    fn it_displays_when_name_is_short() {
        let last_update = SystemTime::now();
        let branch = Branch::new("name".to_string(), true, last_update, "a1b2c3D".to_string());
        assert_eq!(branch.display(), "name            | a1b2c3D | 0 sec")
    }
    
    #[test]
    fn it_displays_when_name_is_long() {
        let last_update = SystemTime::now();
        let branch = Branch::new("aSuperSuperLongName".to_string(), true, last_update, "a1b2c3D".to_string());
        assert_eq!(branch.display(), "aSuperSuperLo...| a1b2c3D | 0 sec")
    }
}
