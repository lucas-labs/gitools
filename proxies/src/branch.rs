use eyre::Result;

const COMMAND: &str = "branch";

/// Proxify the `git branch` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
