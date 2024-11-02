use {
    common::{
        cli::{
            action::{self, Action::*},
            print,
        },
        git,
    },
    eyre::Result,
    lool::s,
};

pub fn handle(command: &str) -> Result<()> {
    let result = match action::get(command) {
        Cmd { cmd, args } => git::run(&cmd, args),
        ShowVersion => print::version(command, env!("CARGO_PKG_VERSION")),
        // try to print tldr or call git {cmd} --help if a custom tldr is not found
        ShowTldr { name } => print::tldr(&name).or_else(|_| git::run(command, vec![s!("--help")])),
    };

    match result {
        Err(_) => std::process::exit(1),
        result => result,
    }
}
