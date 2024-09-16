use eyre::Result;

const COMMAND: &str = "add";

/// Proxify the `git add` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
