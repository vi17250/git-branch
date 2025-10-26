# Git branch

**Use `git-branch` to manage local git branches interactively 🔥**

- Displays the origin branch
- Displays the HEAD branch
- Displays all local branches with the commit hash and time elapsed since the last modification
- Delete branches simultaneously

## Who is it intended for?

For all developers who use git as their version control system

## Installation

### Pre-requisite

1. [Install `rustup`](https://www.rust-lang.org/tools/install) to compile the source code for our device/os

### Usage

1. 📦 Install with cargo from [github repository](https://crates.io/)
```bash 
cargo install git-branch
```
 2. 🥈 Run `git-branch`

 3. 🚀 Optional but incredible: *Add an alias oh my zsh* 
```bash
touch $ZSH_CUSTOM/aliases.zsh
echo "alias ggb=git-branch" >> $ZSH_CUSTOM/aliases.zs
source ~/.zshrc 
```
4. 🥇 Run `ggb`

![demo](https://github.com/user-attachments/assets/2c8ced9f-1941-4a6f-bbdc-091d0639fa68)

## Special thanks:

The original idea came from the 🧙‍♂️ [Valentin Barit](https://github.com/quibaritaenperdresatrompe)

The original repo can be accessed here [https://github.com/quibaritaenperdresatrompe/git-branch](https://github.com/quibaritaenperdresatrompe/git-branch)

