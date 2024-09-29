mod git_config;
pub use git_config::{GitConfig, GitConfigSection, OptionValue};

use {
    crate::cli::{context::ExecutionContext, print},
    eyre::{Context, Result},
    lool::{cli::stylize::stylize, fail, s},
    std::{
        path::PathBuf,
        process::{Command, Stdio},
    },
};

pub enum Head {
    Branch(String),
    Commit(String),
}

#[derive(Debug)]
pub struct Git {
    config: GitConfig,
    head: String,
    root: PathBuf,
}

impl Git {
    /// Create a new Git instance by reading the `.git/config` file from the current working
    /// directory.
    pub fn new(runtime: &ExecutionContext) -> Result<Self> {
        // check if the current working directory is a git repository by checking if the
        // `.git/config` file exists.
        let (root, config) = Self::get_repo(runtime).context("Failed to get git repo info")?;
        let head = Self::read_head_file(&root).context("Failed to read HEAD file")?;

        Ok(Self { config, head, root })
    }

    /// Find and read the `.git/config` file returning its path and content and return the
    /// repository root path.
    ///
    /// Fails if the current working directory is not a git repository.
    fn get_repo(runtime: &ExecutionContext) -> Result<(PathBuf, GitConfig)> {
        let (repo_path, config_path) = Self::discover_paths(&runtime.cwd)?;
        Ok((repo_path, GitConfig::from_path(config_path)?))
    }

    /// Recursively search for a `.git/config` file, starting from the current working directory and
    /// going down (`current/..`) until the root of the file system is reached. This allows the user
    /// to run the command from any subdirectory of the repository. Returns the path to the
    /// root of the repository and the path to the `.git/config` file or an error if it's not found
    /// (not a git repo).
    fn discover_paths(cwd: &PathBuf) -> Result<(PathBuf, PathBuf)> {
        let mut cwd = cwd.clone();

        loop {
            let mut current_dir = cwd.clone();
            current_dir.push(".git/config");

            if current_dir.exists() {
                return Ok((cwd, current_dir));
            }

            // if we reached the root of the file system, break the loop
            if !cwd.pop() {
                break;
            }
        }

        fail!(get_not_a_git_repo_err())
    }

    /// Get the value of the `HEAD` file in the `.git` directory.
    fn read_head_file(root: &PathBuf) -> Result<String> {
        let mut head = root.clone();
        head.push(".git/HEAD");

        let head = std::fs::read_to_string(head)?;
        Ok(s!(head.trim()))
    }

    /// returns the current branch name
    pub fn get_head(&self) -> Head {
        let trimmed_content = self.head.trim();

        if trimmed_content.starts_with("ref: refs/heads/") {
            // On a branch, extract the branch name
            let branch_name = s!(trimmed_content.trim_start_matches("ref: refs/heads/").trim());
            Head::Branch(branch_name)
        } else {
            // In a detached HEAD state, return the commit hash
            Head::Commit(s!(trimmed_content))
        }
    }

    /// get the path to the root of the git repository
    pub fn get_repo_path(&self) -> &PathBuf {
        &self.root
    }

    /// executes a git command and returns the output when the command is done.
    pub fn exec_and_get_result(&self, cmd: &str, args: &[&str]) -> Result<String> {
        let output = std::process::Command::new("git")
            .arg(cmd)
            .args(args)
            .current_dir(&self.config.get_repo_path())
            .output()?;

        if !output.status.success() {
            return fail!(
                "Failed to execute git command: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(s!(String::from_utf8_lossy(&output.stdout)))
    }

    /// Executes a git command and prints the output in real-time.
    /// If the command fails, returns Err.
    pub fn exec(&self, cmd: &str, args: &Vec<String>) -> Result<()> {
        // Spawn the `git` command
        let mut child = Command::new("git")
            .arg(cmd)
            .args(args)
            .current_dir(&self.config.get_repo_path())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Print the output of the child process
        print::child_output(&mut child, format!("git {} {}", cmd, args.join(" ")))?;

        // Wait for the child process to finish and check the status
        let status = child.wait()?;

        // Handle non-success exit statuses
        if !status.success() {
            fail!("Failed to execute git command: {:?}", status)
        } else {
            Ok(())
        }
    }

    /// returns the config of the git repository (`.git/config` file)
    pub fn config(&self) -> &GitConfig {
        &self.config
    }
}

/// executes a git command and prints the output to stdout in real-time as it comes
/// from the process.
/// If the command fails, the error is printed to stderr and Returns Err
pub fn run(cmd: &str, args: Vec<String>) -> Result<()> {
    let mut child = std::process::Command::new("git")
        .arg(cmd)
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        fail!("Failed to execute git command: {:?}", status)
    } else {
        Ok(())
    }
}

// executes a git command and returns the output
pub fn exec(cmd: &str, args: Vec<&str>) -> Result<String> {
    let output = std::process::Command::new("git").arg(cmd).args(args).output()?;

    if !output.status.success() {
        return fail!("Failed to execute git command: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(s!(String::from_utf8_lossy(&output.stdout)))
}

fn get_not_a_git_repo_err() -> String {
    let err = stylize("Could not find a .git/config file", "red");
    let suggestion =
        format!("{} Are you sure you are in a git repository?", stylize("Hint:", "magenta"));

    format!("{}\n\n{}", err, suggestion)
}
