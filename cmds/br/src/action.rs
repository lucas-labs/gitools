use common::git;
use eyre::Result;
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet, Styled},
    Confirm, Select,
};
use lool::{cli::stylize::stylize, fail};

pub fn select_checkout() -> Result<()> {
    // Run `git branch --all --no-color`
    let output = git::exec("branch", vec!["--all", "--no-color"])?;

    // Parse the output to get the list of branches
    let mut branches = Vec::new();
    for line in output.lines() {
        let line = line.trim();
        if line.starts_with('*') {
            // Current branch, skip
            continue;
        } else {
            // Clean up branch names
            let branch_name = line.trim().to_string();
            branches.push(branch_name);
        }
    }

    // Remove duplicates and sort
    // Sort branches with "main" or "master" at the top, then the rest alphabetically
    branches.sort_by(|a, b| {
        match (a.as_str(), b.as_str()) {
            ("main", _) | (_, "main") => {
                if a == "main" {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            ("master", _) | (_, "master") => {
                if a == "master" {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            _ => a.cmp(b), // Regular alphabetical sorting for other branches
        }
    });

    branches.dedup();

    // If no branches found
    if branches.is_empty() {
        println!("No branches found.");
        return Ok(());
    }

    // Use Select prompt
    let ans = Select::new("Select a branch to checkout", branches)
        .with_page_size(10)
        .without_help_message()
        .with_render_config(RenderConfig {
            highlighted_option_prefix: Styled::new(">").with_fg(Color::LightBlue),
            selected_option: Some(
                StyleSheet::new().with_attr(Attributes::ITALIC).with_fg(Color::LightBlue),
            ),
            answer: StyleSheet::new().with_attr(Attributes::ITALIC).with_fg(Color::LightBlue),
            help_message: StyleSheet::new().with_fg(Color::White),
            ..Default::default()
        })
        .prompt();

    match ans {
        Ok(choice) => {
            // Perform `git checkout <choice>`
            let output = git::exec("checkout", vec![&choice])?;
            println!("{}", output);
            Ok(())
        }
        Err(err) => fail!("{}", stylize(err.to_string(), "red")),
    }
}

pub fn checkout(branch: String) -> Result<()> {
    // Try to checkout the branch
    let result = git::exec("checkout", vec![&branch]);

    match result {
        Ok(output) => {
            // Checkout succeeded
            println!("{}", output);
            Ok(())
        }
        Err(_) => {
            // Ask for confirmation to create new branch
            let ans = Confirm::new(&format!("Create a new branch '{}'?", &branch))
                .with_default(true)
                .prompt();

            match ans {
                Ok(true) => {
                    // Create new branch
                    let output = git::exec("checkout", vec!["-b", &branch])?;
                    println!("{}", output);
                    Ok(())
                }
                Ok(false) => {
                    println!("Aborted.");
                    Ok(())
                }
                Err(err) => fail!("{}", stylize(err.to_string(), "red")),
            }
        }
    }
}

pub fn list_branches() -> Result<()> {
    // Run `git branch --all --no-color`
    let output = git::exec("branch", vec!["--all", "--no-color"])?;
    let mut current_branch = "".to_string();

    // Parse the output to get the list of branches
    let mut branches = Vec::new();
    for line in output.lines() {
        let line = line.trim();
        // remove the prefiex "* " if it exists
        if line.starts_with('*') {
            current_branch = line.trim_start_matches("* ").to_string();
            continue;
        }
        let line = line.trim_start_matches("* ");
        branches.push(line.to_string());
    }

    // Remove duplicates and sort
    // Sort branches with "main" or "master" at the top, then the rest alphabetically
    branches.sort_by(|a, b| match (a.as_str(), b.as_str()) {
        ("main", _) | (_, "main") => {
            if a == "main" {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
        ("master", _) | (_, "master") => {
            if a == "master" {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
        _ => a.cmp(b),
    });

    branches.dedup();

    println!("{}", stylize("╭─", "blue+bold"));
    println!(
        "{} {} {}",
        stylize("│", "blue+bold"),
        stylize(current_branch, "blue+bold"),
        stylize("(current)", "+dim")
    );
    for branch in branches {
        println!("{} {}", stylize("│", "blue+bold"), branch);
    }
    println!("{}", stylize("╰─", "blue+bold"));

    Ok(())
}

pub fn delete(branch: String) -> Result<()> {
    // Try to delete the branch
    let result = git::exec("branch", vec!["-d", &branch]);

    match result {
        Ok(output) => {
            // Deletion succeeded
            println!("{}", output);
            Ok(())
        }
        Err(_) => fail!("{}", stylize(format!("Branch {} does not exist.", branch), "red")),
    }
}
