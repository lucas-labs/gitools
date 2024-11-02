use {eyre::Result, lool::cli::stylize::stylize};

/// print the version of the binary
pub fn print_version(command: &str, version: &str) -> Result<()> {
    println!("{}{}{}", stylize(command, "blue"), stylize("@", "+dim"), version);
    Ok(())
}
