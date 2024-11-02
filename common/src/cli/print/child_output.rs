use {
    eyre::Result,
    lool::cli::stylize::stylize,
    std::{
        io::{BufRead, BufReader},
        process::Child,
    },
};

/// Generic function to print a child process's output (stdout and stderr) in a box
pub fn print_child_output(child: &mut Child, command: String) -> Result<()> {
    // Print the start of the box
    println!(
        "{} {} {}",
        stylize("╭─", "bright-cyan+bold"),
        stylize("start:", "magenta"),
        stylize(&command, "bright-cyan+bold")
    );
    println!("{}", stylize("│", "bright-cyan+bold"));

    // Capture both stdout and stderr
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    // Print stdout lines
    for line in stdout_reader.lines() {
        if let Ok(line) = line {
            // strip \r\x1b[K from the line if present in any part of the line
            let line = line.replace("\r\x1b[K", "");
            println!("{}  {}", stylize("│", "bright-cyan+bold"), line); // Indicate stdout output
        }
    }

    // Print stderr lines (if any)
    for line in stderr_reader.lines() {
        if let Ok(line) = line {
            let line = line.replace("\r\x1b[K", "");
            println!("{}  {}", stylize("│", "red+bold"), line); // Indicate stderr output
        }
    }

    println!("{}", stylize("│", "bright-cyan+bold"));
    println!(
        "{} {} {}",
        stylize("╰─", "bright-cyan+bold"),
        stylize("end:", "magenta"),
        stylize(&command, "bright-cyan+bold")
    );

    Ok(())
}
