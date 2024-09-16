use {eyre::Result, lool::cli::stylize::stylize};

/// print the version of the binary
pub fn print_version(command: &str) -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    println!("{}{}{}", stylize(command, "blue"), stylize("@", "+dim"), version);
    Ok(())
}
