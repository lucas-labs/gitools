use eyre::Result;

const COMMAND: &str = "commit";

/// Proxify the `git commit` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
