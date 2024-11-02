use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

use lool::cli::stylize::stylize;

fn is_bat_available() -> Option<String> {
    // check if bat.exe is in the path
    let mut found = false;
    let mut path = String::new();

    if let Ok(paths) = env::var("PATH") {
        for p in paths.split(';') {
            let p = format!("{}/bat.exe", p);
            if std::fs::metadata(&p).is_ok() {
                found = true;
                path = p;
                break;
            }
        }
    }

    if found {
        Some(path)
    } else {
        None
    }
}

/// prints the context of a file to the terminal using `bat`
/// run bat.exe - {stdin} as a command and let's it's otput show in real-time
///
/// if bat is not available, it will fallback to `cat`
///
///
pub fn print_bat(file_path: &str, content: String, lang: &str) {
    let path;

    let maybe_bat_exe = is_bat_available();
    if let Some(bat_path) = &maybe_bat_exe {
        path = Some(bat_path);
    } else {
        return print_cat(file_path, &content);
    }

    let styles = ["numbers", "grid", "header"];

    // Spawn the bat.exe process with inherited stdin and stdout
    let mut process = Command::new(path.unwrap())
        .arg("-l")
        .arg(lang)
        .arg("--file-name")
        .arg(file_path)
        .arg("--paging=never")
        .arg("--style")
        .arg(styles.join(","))
        .stdin(Stdio::piped()) // Use a pipe for stdin to send input
        .stdout(Stdio::inherit()) // Inherit stdout to write output to the console
        .spawn()
        .expect("Failed to start bat.exe");

    // Write the input to the process's stdin
    if let Some(mut stdin) = process.stdin.take() {
        // std::thread::spawn(move || {
        stdin.write_all(content.as_bytes()).expect("Failed to write to stdin");
        // });
    }

    // Wait for the process to finish
    let _ = process.wait().expect("Failed to wait on bat.exe");
}

/// prints the content of a file to the terminal with a file name header
pub fn print_cat(file_path: &str, content: &str) {
    // Print the file name as a header
    println!("{}", stylize(file_path, "+dim"));
    println!("{}\n", stylize("─".repeat(file_path.len()), "+dim"));

    // Print the content
    println!("{}", content);
}
