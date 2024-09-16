use eyre::Result;

const COMMAND: &str = "merge";

/// Proxify the `git merge` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
