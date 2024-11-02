use {
    eyre::Result,
    lool::{cli::stylize::stylize, fail},
};

pub enum Action {
    Version,
    Help,
    List,
    Add,
    Remove,
    Set,
    View,
    ShowConfig,
}

pub fn parse_args() -> Result<Action> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        return Ok(Action::Help);
    }

    if pargs.contains(["-v", "--version"]) {
        return Ok(Action::Version);
    }

    let maybe_subcmd = pargs.subcommand();

    match maybe_subcmd {
        Ok(Some(subcmd)) => match subcmd.as_str() {
            "list" => Ok(Action::List),
            "add" => Ok(Action::Add),
            "rm" => Ok(Action::Remove),
            "set" => Ok(Action::Set),
            "cfg" => Ok(Action::ShowConfig),
            _ => f(),
        },
        Ok(None) => Ok(Action::View),
        _ => f(),
    }
}

fn f() -> Result<Action> {
    fail!(
        "{}\nTry {} for more information",
        stylize("No subcommand provided", "red"),
        stylize("usr --help", "yellow+italic")
    )
}
