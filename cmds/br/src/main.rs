//! # git br
//!
//! This command is a git utility to manage branches.
//!
//! It is a wrapper around `git branch` and `git checkout` commands.
//!
//! - When you call `br` without any arguments, it will list all the branches in the repository in a
//! Select prompt where you can navigate with up and down arrows and select a branch with the Enter
//! key. When selecting a branch, it will perform a `git checkout <branch>` command.
//!
//! - If you call `br` with a branch name, like `br my-branch`, it will first check if the branch
//! exists. If it does, it will perform a `git checkout <branch>` command. If it doesn't, it will
//! perform a `git checkout -b <branch>` command and create a new branch (asking first for your
//! confirmation).
//!
//! - If you call `br --list or -l`, it will list all the branches in the repository in a Select

use {
    args::{parse_args, Action},
    common::cli::print,
    eyre::Result,
};

mod action;
mod args;

const COMMAND: &str = "br";

fn main() -> Result<()> {
    match parse_args()? {
        Action::Version => print::version(COMMAND, env!("CARGO_PKG_VERSION")),
        Action::Help => print::tldr(COMMAND),
        Action::Checkout => action::select_checkout(),
        Action::CheckoutBranch(branch) => action::checkout(branch),
        Action::List => action::list_branches(),
        Action::DeleteBranch(branch) => action::delete(branch),
    }
}
