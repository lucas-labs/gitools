use eyre::Result;

const COMMAND: &str = "pull";

/// Proxify the `git pull` command.
fn main() -> Result<()> {
    proxies::handle(COMMAND)
}
