use eyre::Result;

const COMMAND: &str = "checkout";

/// Proxify the `git checkout` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
