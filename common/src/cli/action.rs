use lool::s;

pub enum Action {
    ShowTldr { name: String },
    ShowVersion,
    Cmd { cmd: String, args: Vec<String> },
}

/// determine the action to take based on the command and arguments passed to the binary
pub fn get(command: &str) -> Action {
    // get the arguments passed to the binary and check if its --help -h, -v or --version
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help" || arg == "tldr" || arg == "--tldr") {
        return Action::ShowTldr { name: s!(command) };
    }

    if args.iter().any(|arg| arg == "-v" || arg == "--version") {
        return Action::ShowVersion;
    }

    Action::Cmd {
        cmd: s!(command),
        args,
    }
}
