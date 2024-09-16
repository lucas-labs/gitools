use eyre::Result;

const COMMAND: &str = "status";

/// Proxify the `git status` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
