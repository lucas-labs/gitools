//! # git usr
//!
//! This command is a git utility to manage git user settings as profiles.
//!
//! It stores "user profiles" (name, email, signingkey) in a TOML file called `gusr.toml` in the
//! same directory as the executable.
//!
//! The user can add, list, and delete profiles to the `gusr.toml` file:
//!
//! - `usr`: show the current user profile
//! - `usr list`: list all user profiles
//! - `usr add`: add a new user profile
//! - `usr rm`: remove a user profile
//!
//! And most importantly, the user can switch between profiles:
//!
//! - `usr set`: display a selectable list of profiles to switch to. Uses `git config --global` to
//!   set the `user.name`, `user.email`, and `user.signingkey` to match the selected profile.
//!
//!   It also sets a custom git configuration `gusrs.active` to the selected profile ID so that it
//!   can then be used to know which profile is currently active.

use {
    args::{parse_args, Action},
    common::cli::print,
    eyre::Result,
};

mod action;
mod args;
mod profiles;

const COMMAND: &str = "gusr";

fn main() -> Result<()> {
    match parse_args()? {
        Action::Version => print::version(COMMAND, env!("CARGO_PKG_VERSION")),
        Action::Help => print::tldr(COMMAND),
        Action::Add => action::add(),
        Action::List => action::list(),
        Action::ShowConfig => action::show_config(),
        Action::Remove => action::remove(),
        Action::Set => action::set(),
        Action::View => action::view(),
    }
}
