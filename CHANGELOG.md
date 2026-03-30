# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5](https://github.com/vi17250/git-branch/compare/v0.1.4...v0.1.5) - 2026-03-30

### Fixed

- 🐛 remove `Origin` feature

### Other

- *(deps)* 🤖 update to valinta 0.2.0
- 💡 create a `file_system` module which find .git dir

## [0.1.4](https://github.com/vi17250/git-branch/compare/v0.1.3...v0.1.4) - 2026-03-29

### Other

- Revert "chore(deps): 🤖 update valinta to v 0.2.0"

## [0.1.3](https://github.com/vi17250/git-branch/compare/v0.1.2...v0.1.3) - 2026-03-29

### Fixed

- 🐛 Throw an error and quit if there are no existing commit
- 🐛 change message when there are no branches to delete
- 🐛 change message when it's not a git repository

### Other

- *(deps)* 🤖 update valinta to v 0.2.0
- 💡 some wording and optimization

## [0.1.2](https://github.com/vi17250/git-branch/compare/v0.1.1...v0.1.2) - 2025-12-15

### Fixed

- 🐛 throw error when origin not exist

## [0.1.1](https://github.com/vi17250/git-branch/compare/v0.1.0...v0.1.1) - 2025-12-10

### Other

- 💡 uses valinta as multi selector
- 🤖 changelog uses the right repo
- ✏️ remove github link
- 🤖 remove useless crate `regex`
- release v0.1.0

## [0.1.0](https://github.com/vi17250/git-branch/releases/tag/v0.1.0) - 2025-10-26

### Added

- 🎸 display commit msg
- 🎸 prevent origin deletion
- 🎸 display origin
- 🎸 display commit hash and time of HEAD
- 🎸 display HEAD branch
- 🎸 display last update in seconds
- 🎸 display commit hash
- 🎸 Commit is a struct included in Branch
- 🎸 delete branches 🔥
- 🎸 multiple selection for branches to delete
- 🎸 add last update to Branch struct
- 🎸 set head branch
- 🎸 find refs directory and create branches
- 🎸 find .git directory
- 🎸 initial_commit

### Fixed

- 🐛 restore commit hash
- 🐛 add test config
- 🐛 display last commit update time
- 🐛 display last commit update time
- 🐛 parse logs and filter only commits
- 🐛 exit when there are no branches to delete
- 🐛 remove branches whose names include slashes
- 🐛 detect when branches contains slashes (e.g. branch1/a)
- 🐛 get_branches() display error message
- 🐛 it detects when no origin exist
- 🐛 remove useless import
- 🐛 display HEAD even if it's origin
- 🐛 displays the human-readable modification time
- 🐛 The selection cannot go beyond the list of branches
- 🐛 dialog interface highlight branch
- 🐛 returns an error if no git repository was found
- 🐛 display message when no branches are found
- 🐛 returns all branches without HEAD
- 🐛 Exit process
- 🐛 add logs_dir in Branch struct

### Other

- 🎡 re-activate release
- 🤖 update version number for production
- ✏️ wording
- 💡 is_empty() is clearer and more explicit than .len()
- demo is a gif
- presentation video
- 💡 display error when no commit exists
- ✏️ wording
- 💡 good practices, thank you clippy
- ✏️ describe features / details the installation procedure
- 💡 error domain
- 🎡 disable release
- ✏️ install from cargo
- release v0.1.2
- 🤖 semver true
- release v0.1.1
- 🤖 todolist is not commited
- ✏️ drop waiting message
- release v0.1.0
- 🤖 V0.1.0 🥳
- 🤖 Cargo.toml display metadata fields
- release v0.0.1
- 🎡 release with release-plz
- 🤖 cliff configuration for CHANGELOG generation
- 🎡 build and test
- ✏️ todo
- ✏️ who is it useful for?
- ✏️ informations about git-branch
- 💡 errors handling
- Update version in Cargo.toml
- 💡 HEAD domain
- 🤖 rename crate
- ✏️ project is not yet ready
- 💡 the folder structure is organized by subject area
- 💍 branches filter
- ✏️ idea from the sorcerer

## [0.1.2](https://github.com/vi17250/git-branch/releases/tag/v0.1.2) - 2025-09-26

### Added

- 🎸 display origin
- 🎸 display commit hash and time of HEAD
- 🎸 display HEAD branch
- 🎸 display last update in seconds
- 🎸 display commit hash
- 🎸 Commit is a struct included in Branch
- 🎸 delete branches 🔥
- 🎸 multiple selection for branches to delete
- 🎸 add last update to Branch struct
- 🎸 set head branch
- 🎸 find refs directory and create branches
- 🎸 find .git directory
- 🎸 initial_commit

### Fixed

- 🐛 it detects when no origin exist
- 🐛 remove useless import
- 🐛 display HEAD even if it's origin
- 🐛 displays the human-readable modification time
- 🐛 The selection cannot go beyond the list of branches
- 🐛 dialog interface highlight branch
- 🐛 returns an error if no git repository was found
- 🐛 display message when no branches are found
- 🐛 returns all branches without HEAD
- 🐛 Exit process
- 🐛 add logs_dir in Branch struct

### Other

- 🤖 semver true
- release v0.1.1
- 🤖 todolist is not commited
- ✏️ drop waiting message
- release v0.1.0
- 🤖 V0.1.0 🥳
- 🤖 Cargo.toml display metadata fields
- release v0.0.1
- 🎡 release with release-plz
- 🤖 cliff configuration for CHANGELOG generation
- 🎡 build and test
- ✏️ todo
- ✏️ who is it useful for?
- ✏️ informations about git-branch
- 💡 errors handling
- Update version in Cargo.toml
- 💡 HEAD domain
- 🤖 rename crate
- ✏️ project is not yet ready
- 💡 the folder structure is organized by subject area
- 💍 branches filter
- ✏️ idea from the sorcerer

## [0.1.1](https://github.com/vi17250/git-branch/compare/v0.1.0...v0.1.1) - 2025-09-25

### Other

- 🤖 todolist is not commited
- ✏️ drop waiting message

## [0.1.0](https://github.com/vi17250/git-branch/releases/tag/v0.1.0) - 2025-09-25

### Added

- 🎸 display origin
- 🎸 display commit hash and time of HEAD
- 🎸 display HEAD branch
- 🎸 display last update in seconds
- 🎸 display commit hash
- 🎸 Commit is a struct included in Branch
- 🎸 delete branches 🔥
- 🎸 multiple selection for branches to delete
- 🎸 add last update to Branch struct
- 🎸 set head branch
- 🎸 find refs directory and create branches
- 🎸 find .git directory
- 🎸 initial_commit

### Fixed

- 🐛 remove useless import
- 🐛 display HEAD even if it's origin
- 🐛 displays the human-readable modification time
- 🐛 The selection cannot go beyond the list of branches
- 🐛 dialog interface highlight branch
- 🐛 returns an error if no git repository was found
- 🐛 display message when no branches are found
- 🐛 returns all branches without HEAD
- 🐛 Exit process
- 🐛 add logs_dir in Branch struct

### Other

- 🤖 V0.1.0 🥳
- 🤖 Cargo.toml display metadata fields
- release v0.0.1
- 🎡 release with release-plz
- 🤖 cliff configuration for CHANGELOG generation
- 🎡 build and test
- ✏️ todo
- ✏️ who is it useful for?
- ✏️ informations about git-branch
- 💡 errors handling
- Update version in Cargo.toml
- 💡 HEAD domain
- 🤖 rename crate
- ✏️ project is not yet ready
- 💡 the folder structure is organized by subject area
- 💍 branches filter
- ✏️ idea from the sorcerer

## [0.0.1](https://github.com/vi17250/git-branch/releases/tag/v0.0.1) - 2025-09-25

### Added

- 🎸 display origin
- 🎸 display commit hash and time of HEAD
- 🎸 display HEAD branch
- 🎸 display last update in seconds
- 🎸 display commit hash
- 🎸 Commit is a struct included in Branch
- 🎸 delete branches 🔥
- 🎸 multiple selection for branches to delete
- 🎸 add last update to Branch struct
- 🎸 set head branch
- 🎸 find refs directory and create branches
- 🎸 find .git directory
- 🎸 initial_commit

### Fixed

- 🐛 remove useless import
- 🐛 display HEAD even if it's origin
- 🐛 displays the human-readable modification time
- 🐛 The selection cannot go beyond the list of branches
- 🐛 dialog interface highlight branch
- 🐛 returns an error if no git repository was found
- 🐛 display message when no branches are found
- 🐛 returns all branches without HEAD
- 🐛 Exit process
- 🐛 add logs_dir in Branch struct

### Other

- 🎡 release with release-plz
- 🤖 cliff configuration for CHANGELOG generation
- 🎡 build and test
- ✏️ todo
- ✏️ who is it useful for?
- ✏️ informations about git-branch
- 💡 errors handling
- Update version in Cargo.toml
- 💡 HEAD domain
- 🤖 rename crate
- ✏️ project is not yet ready
- 💡 the folder structure is organized by subject area
- 💍 branches filter
- ✏️ idea from the sorcerer
