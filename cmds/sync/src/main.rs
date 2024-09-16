//! # git sync
//!
//! This command is a wrapper around:
//!  - `git fetch <remote>`
//!  - `git rebase <remote>/<branch>`
//!  - and optionally `git push <another-remote> <branch>`
//!
//! It's intended to be used in a context where you have two or more remotes and you want to sync
//! your local copy with one of them without merge commits. The optional push is useful when you
//! want to sync your local copy with one remote and push the changes to another remote.
//!
//! In summary, this command is useful to keep 3 repositories in sync without merge commits.
//!
//! ## usage:
//!
//! **Case**: Get changes from `gitea` into local copy.
//!
//! `sync from gitea`
//!
//! **where**:
//!   - `sync` is the command
//!   - `from` is a subcommand, just for semantic reasons
//!   - `gitea` is the name of the remote you want to sync your local copy with
//!
//! **and will**:
//!   - `git fetch gitea`
//!   - `git rebase gitea/master` (assuming you are in the `master` branch)
//!
//! **Case**: Get changes from `gitea` into local copy and push to `github`
//!
//! `sync from gitea to github`
//!
//! **where**:
//!   - `sync` is the command
//!   - `from` is a subcommand, just for semantic reasons
//!   - `gitea` is the name of the remote you want to sync your local copy with
//!   - `to` is a subcommand, just for semantic reasons
//!   - `github` is the name of the remote you want to push the changes to
//!
//! **and will**:
//!   - `git fetch gitea`
//!   - `git rebase gitea/master` (assuming you are in the `master` branch)
//!   - `git push github master`
//!
//! **Case**: Get a new branch from `github`
//!
//! `sync from github:feature-branch`
//!
//! **where**:
//!   - `sync` is the command
//!   - `from` is a subcommand, just for semantic reasons
//!   - `github:feature/branch` is the remote and branch you want to sync your local copy with
//!
//! **and will**:
//!   - `git fetch github feature/branch`
//!   - `git checkout feature/branch` (this will create a new branch if it doesn't exist)
//!   - `git rebase github/feature/branch`
//!   
//! **Case**: Get a new branch from `github` and push to `gitea`
//!
//! `sync from github:feature-branch to gitea`
//!
//! **where**:
//!   - `sync` is the command
//!   - `from` is a subcommand, just for semantic reasons
//!   - `github:feature/branch` is the remote and branch you want to sync your local copy with
//!   - `to` is a subcommand, just for semantic reasons
//!   - `gitea` is the name of the remote you want to push the changes to
//!
//! **and will**:
//!   - `git fetch github feature/branch`
//!   - `git checkout feature/branch` (this will create a new branch if it doesn't exist)
//!   - `git rebase github/feature/branch`
//!   - `git push gitea feature/branch`
//!
//! ## Note:
//!
//! Before running all the commands, this program will show you the commands that will be executed
//! and ask for your confirmation. If you don't confirm, the program will exit without running any
//! command.

mod plan;

use {
    common::{
        cli::{
            action::{self, Action::*},
            context::ExecutionContext,
            print,
        },
        git::{Git, Head},
    },
    eyre::{Ok, Result},
    lool::{cli::stylize::stylize, fail},
    plan::Plan,
    std::io::{self, Write},
};

const COMMAND: &str = "sync";

fn main() -> Result<()> {
    match action::get(COMMAND) {
        ShowTldr { name } => print::tldr(&name),
        ShowVersion => print::version(COMMAND),
        Cmd { cmd: _, args } => {
            check_args(&args)?;
            let git = Git::new(&ExecutionContext::new()?)?;

            match git.get_head() {
                Head::Branch(branch) => handle(args, branch, &git),
                Head::Commit(commit) => {
                    fail!("You are in a detached HEAD state at commit {}", commit)
                }
            }
        }
    }
}

fn handle(args: Vec<String>, branch: String, git: &Git) -> Result<()> {
    let plan = Plan::from_args(args, branch, Some(git.config().get_remotes()))?;

    println!("Will execute the following commands: \n");
    plan.print(Some(&stylize("  - ", "green")));

    if confirm("Do you want to continue?").is_ok() {
        println!("\n{}\n", stylize("Executing commands...", "cyan"));

        let commands = plan.commands();
        for (i, (cmd, args)) in commands.iter().enumerate() {
            git.exec(cmd, &args)?;

            if i < commands.len() - 1 {
                println!();
            }
        }

        println!("\n{}", stylize("Done", "green"));

        Ok(())
    } else {
        eprintln!("\n{}\n", stylize("Aborted", "red"));
        // exit with a non-zero status code
        std::process::exit(1);
    }
}

fn confirm(msg: &str) -> Result<()> {
    // Print the prompt message without a newline
    print!(
        "\n{} {} {}",
        stylize("❱", "green"),
        msg,
        stylize("Y/N (default is Y): ", "+dim")
    );
    io::stdout().flush()?; // Flush to ensure the message is printed immediately

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?; // Read user input

    // Remove the trailing newline from the input
    let input = input.trim().to_lowercase();

    // If the input is empty or 'y', consider it as confirmation
    if input.is_empty() || input == "y" {
        // Move the cursor back to overwrite the user's input 'y' with the formatted output
        print!("\x1b[1A\x1b[2K"); // ANSI escape to move up and clear the line
        print!("\r{} {} {}\n", stylize("❱", "green"), msg, stylize("Y", "green"));
        io::stdout().flush()?; // Flush again to ensure it appears immediately

        Ok(())
    } else {
        print!("\x1b[1A\x1b[2K"); // ANSI escape to move up and clear the line
        print!("\r{} {} {}\n", stylize("❱", "green"), msg, stylize("N", "red"));
        io::stdout().flush()?; // Flush again to ensure it appears immediately

        fail!("Aborted")
    }
}

fn check_args(args: &Vec<String>) -> Result<()> {
    let err = fail!("Usage: sync from <remote[:branch]> [to <remote>]");

    if args.is_empty() || args[0] != "from" || args.len() < 2 {
        return err;
    }

    if args.len() > 2 && (args[2] != "to" || args.len() < 4) {
        return err;
    }

    // if both remotes are the same, it doesn't make sense to sync
    if args.len() > 3 && args[1] == args[3] {
        return fail!("Both remotes are the same");
    }

    return Ok(());
}
