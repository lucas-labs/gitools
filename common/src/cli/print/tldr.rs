use {crate::cli::print::md, eyre::Result};

/// get the tldr from ./tldr/{name}.md relative to the executable path
pub fn print_tldr(command: &str) -> Result<()> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| eyre::eyre!("Failed to get parent dir"))?;
    let tldr_path = exe_dir.join(format!("tldr/{}.md", command));
    let tldr = std::fs::read_to_string(tldr_path)?;
    md(&tldr);
    Ok(())
}
