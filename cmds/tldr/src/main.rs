//! # git-tldr
//!
//! A simple CLI tool to display a small `tldr` info for gitools commands.
//!
//! It looks at the workspaces `Cargo.toml` file to find all the members ant their generated
//! binaries and then lists them in the help message.
use {common::cli::print::md, std::path::PathBuf};

include!(concat!(env!("OUT_DIR"), "/binaries_info.rs"));

fn main() {
    let current_exe_home: PathBuf = {
        let p = std::env::current_exe().unwrap();
        p.parent().unwrap().to_path_buf()
    };

    println!("Current exe home: {:?}", &current_exe_home);

    let mut tldrs = vec![];

    for bin in BINARIES {
        let tldr_path = current_exe_home.join("tldr").join(format!("{}.md", bin.name));

        if !tldr_path.exists() {
            continue;
        }

        let tldr_content = std::fs::read_to_string(tldr_path).unwrap();
        let tldr = tldr_content
            .lines()
            .filter(|l| l.starts_with(">"))
            .map(|l| format!("{}", l))
            .collect::<Vec<String>>();

        tldrs.push((bin.name.to_string(), tldr));
    }

    let mut message = String::new();

    // start constructing the message
    message.push_str("# git-tldr\n\n");
    message.push_str("The following gitools are available:\n\n");

    for (name, tldr) in &tldrs {
        message.push_str(&format!("## {}\n", name));
        for line in tldr {
            message.push_str(&format!("{}\n", line));
        }
        message.push_str("\n");
    }

    if tldrs.is_empty() {
        println!("No tldr found for any command.");
        // exit with err
        std::process::exit(1);
    }

    // remove the last \n if present
    message.pop();

    // print the message
    md(&message);
}
