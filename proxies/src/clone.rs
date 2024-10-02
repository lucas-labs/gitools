use eyre::Result;

const COMMAND: &str = "clone";

/// Proxify the `git clone` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
