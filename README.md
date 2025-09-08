Use `git-branch` to manage git branches on interactive mode.

## Pre-requisites

1. [Install `rustup`](https://www.rust-lang.org/tools/install) to compile the source code for our device/os

## Install

1. Clone this repository:

   ```bash
   git clone `https://github.com/vi17250/git-branch`
   ```
    ```bash
    cd git-branch
    ```

2. Build `git-branch` binary 
    ```bash
    cargo build
    ```

3. Install `git-branch` binary:

   ```bash
    cp ./target/debug/git-branch /opt/git-branch
   ```

4. Run `git-branch` from everywhere on your computer 
    ```bash
     echo export PATH=$PATH:/opt/git-branch/ >> {PATH/TO/YOUR/.BASHRC_FILE}
    ```

## Special thanks:

The original idea came from the 🧙‍♂️ [Valentin Barit](https://github.com/quibaritaenperdresatrompe)

The original repo can be accessed here [https://github.com/quibaritaenperdresatrompe/git-branch](https://github.com/quibaritaenperdresatrompe/git-branch)
