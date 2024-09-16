use eyre::Result;

const COMMAND: &str = "log";

/// Proxify the `git log` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
