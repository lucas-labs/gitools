use eyre::Result;

const COMMAND: &str = "push";

/// Proxify the `git push` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
