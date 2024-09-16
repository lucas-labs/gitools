use {
    common::{cli::context::ExecutionContext, git::Git},
    eyre::Result,
    lool::s,
};

fn main() -> Result<()> {
    // Create a dummy config for testing
    let config = ExecutionContext::new()?;

    // Instantiate the Git struct with the config
    let git = Git::new(&config)?;

    // Execute a git command and print the output
    git.exec("status", &vec![s!("-s")])?;
    println!();
    git.exec("log", &Vec::new())?;

    Ok(())
}
