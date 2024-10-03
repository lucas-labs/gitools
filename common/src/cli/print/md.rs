//! # cli/print/md
//!
//! Print markdown content to the terminal, nicely formatted with colors and sh*t

use {lool::cli::stylize::stylize, regex_lite::Regex};

/// **Print Markdown** content
///
/// This function will print the content of a markdown text to the terminal, nicely formatted with
/// colors.
pub fn print_md(content: &str) {
    // format the content with colors and sh*t
    let formatted_content = get_formatted_content(content);

    // print the content to the terminal
    println!("{}", formatted_content);
}

fn get_formatted_content(content: &str) -> String {
    let content = content.replace("\r\n", "\n");

    let code_block_re = Regex::new(r"(?s)```(\w*)\n(.*?)\n```").unwrap();
    let header_re = Regex::new(r"^(#{1,6})\s*(.+)").unwrap();
    let list_re = Regex::new(r"^(\s*[-*])\s+(.+)").unwrap();
    let link_re = Regex::new(r"\[(.*?)\](?:\((.*?)\))?").unwrap();
    let bold_re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let italic_re = Regex::new(r"\*(.*?)\*").unwrap();
    let code_re = Regex::new(r"`(.*?)`").unwrap();
    let quote_re = Regex::new(r"^(\s*)>s*(.+)").unwrap();

    let formatted_content = code_block_re.replace_all(&content, |caps: &regex_lite::Captures| {
        let language = caps.get(1).map_or("", |m| m.as_str());
        let code = &caps[2];

        let mut formatted_code = String::new();

        // Add language tag if present
        if !language.is_empty() {
            formatted_code.push_str(&format!("{}\n", stylize(language, "+bold")));
        }

        for (i, line) in code.lines().enumerate() {
            let line_number = i + 1; // Line number starts from 1
            let leading_spaces = line.chars().take_while(|&c| c == ' ').count(); // Count leading spaces
            let indent_dots = stylize("·".repeat(leading_spaces / 2), "+dim"); // Convert spaces to middle dots
            let trimmed_line = line.trim_start(); // Remove leading spaces for formatting
            let formatted_line = stylize(trimmed_line, "bright-yellow"); // Apply styling to the line

            // Add line number, middle dots for the indentation, and formatted line
            formatted_code.push_str(&format!(
                "{}{}{}{}",                                   // Format line number and indentation
                stylize(format!("{} ", line_number), "+dim"), // Line number
                indent_dots,                                  // Middle dots for indentation
                formatted_line,                               // Styled line content
                if i == code.lines().count() - 1 {
                    ""
                } else {
                    "\n"
                }  // Add newline if this isn't the last line
            ));
        }

        formatted_code
    });

    let mut final_output = String::new();
    let mut current_indent = 0;

    for line in formatted_content.lines() {
        let mut line = line.to_string();

        // Check if the line is a header
        if let Some(caps) = header_re.captures(&line) {
            let header_level = caps[1].len(); // Number of # symbols
            let header_indent = (header_level - 1) * 2; // 2 spaces per level minus 2 for the header itself
            line = " ".repeat(header_indent) + &stylize(&caps[2], "white+bold");
            current_indent = header_level * 2; // Set indentation for subsequent content
        } else {
            // Apply the current indentation to all non-header lines
            line = " ".repeat(current_indent) + &line;
        }

        // Lists
        line = list_re
            .replace_all(&line, |caps: &regex_lite::Captures| {
                let bullet = stylize(&caps[1], "bright-blue");
                let text = &caps[2];
                format!("{} {}", bullet, text)
            })
            .to_string();

        // Links
        line = link_re
            .replace_all(&line, |caps: &regex_lite::Captures| {
                let link = caps.get(2).map_or(&caps[1], |m| m.as_str());
                stylize(format!("<{}>", link), "bright-green")
            })
            .to_string();

        // Bold text
        line = bold_re
            .replace_all(&line, |caps: &regex_lite::Captures| stylize(&caps[1], "+bold"))
            .to_string();

        // Italic text
        line = italic_re
            .replace_all(&line, |caps: &regex_lite::Captures| stylize(&caps[1], "+italic"))
            .to_string();

        // Inline code
        line = code_re
            .replace_all(&line, |caps: &regex_lite::Captures| stylize(&caps[1], "bright-yellow"))
            .to_string();

        // Quotes
        line = quote_re
            .replace_all(&line, |caps: &regex_lite::Captures| {
                format!(
                    "{}{}{}",
                    &caps[1],
                    stylize("“", "bright-cyan+dim"),
                    stylize(&caps[2], "bright-cyan")
                )
            })
            .to_string();

        final_output.push_str(&line);
        final_output.push('\n');
    }

    final_output
}
