use {
    eyre::Result,
    lool::cli::stylize::stylize,
    std::{collections::HashMap, process::Command},
};

const COMMAND: &str = "remote";

/// Proxify the `git remote` command.
///
/// ### Extended functionality
/// This command also extends the `git remote` command adding the following subcommands:
///
/// - `remote list`: list all the remotes in the repository in a pretty format.
fn main() -> Result<()> {
    // extensions

    // check if the user wants to list the remotes by checking the first argument with env::args()
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "list" {
            return list_remotes();
        }
    }

    proxies::handle(COMMAND)
}

fn list_remotes() -> Result<()> {
    // Run the git remote -vv command and get the output
    let output = Command::new("git").arg("remote").arg("-vv").output()?;

    // Check if the command was successful
    if !output.status.success() {
        eprintln!("Failed to execute git remote -vv: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // Parse the output as a string
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Group URLs by remote name
    let mut remotes: HashMap<String, Vec<String>> = HashMap::new();
    for line in output_str.lines() {
        // Split the line into remote name, URL, and fetch/push info
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let name = parts[0].to_string();

            // if fetch, green, if push, blue
            let (dir, color) = if parts[2] == "(fetch)" {
                ("fetch", "green+bold")
            } else {
                ("push ", "red+bold")
            };

            let url = format!("{} {}", stylize(dir, color), parts[1]);

            // Insert the URL into the hashmap grouped by the remote name
            remotes.entry(name).or_insert_with(Vec::new).push(url);
        }
    }

    let colors = ["blue+bold", "bright-cyan+bold", "bright-magenta+bold"];

    // Print each remote name with its URLs in a box format
    let total_remotes = remotes.len();
    for (i, (remote, urls)) in remotes.iter().enumerate() {
        let col = colors[i % colors.len()];
        println!("{} {}", stylize("╭─", &col), stylize(&remote, &col));
        for url in urls {
            println!("{} {}", stylize("│", &col), url);
        }
        println!("{}", stylize("╰─", &col));

        // Print a newline unless it's the last remote
        if i < total_remotes - 1 {
            println!();
        }
    }
    Ok(())
}
