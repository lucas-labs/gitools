// run command `ping` and print the output in real-time as it comes but wrap the result in a box
// like:

// ```bash
// ╭─ ping
// │ line 1
// │ line 2
// │ line 3
// │ line 4
// │ line 5
// │ line 6
// ╰─ end ping

use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    // Example usage of the function
    run_command_in_box("ping", &["8.8.8.8"]);
}

fn run_command_in_box(command: &str, args: &[&str]) {
    // Spawn the provided command with the given arguments
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped()) // Capture stdout
        .spawn()
        .expect("Failed to execute command");

    // Print the start of the box
    println!("╭─ {}", command);

    // Get the stdout of the child process
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                // Print each line with a vertical box line
                println!("│ {}", line);
            }
        }
    }

    // Wait for the child process to finish
    child.wait().expect("Child process wasn't running");

    // Print the end of the box
    println!("│\n╰─ end {}", command);
}
