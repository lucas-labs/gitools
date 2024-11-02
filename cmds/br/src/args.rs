use {
    eyre::Result,
    lool::{cli::stylize::stylize, fail},
};

pub enum Action {
    Version,
    Help,
    List,
    Checkout,
    CheckoutBranch(String),
    DeleteBranch(String),
}

pub fn parse_args() -> Result<Action> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        return Ok(Action::Help);
    }

    if pargs.contains(["-v", "--version"]) {
        return Ok(Action::Version);
    }

    if pargs.contains(["-l", "--list"]) {
        return Ok(Action::List);
    }

    if let Ok(branch) = pargs.value_from_str(["-d", "--delete"]) {
        return Ok(Action::DeleteBranch(branch));
    }

    if let Ok(branch) = pargs.free_from_str::<String>() {
        if branch.trim().starts_with('-') {
            return fail!(
                "{}\nTry {} for more information",
                stylize("Bad Usage", "red"),
                stylize("br --help", "yellow+italic")
            );
        }

        return Ok(Action::CheckoutBranch(branch));
    }

    // ensure not other arguments are passed
    let x = pargs.finish();

    // if x is empty
    if !x.is_empty() {
        fail!(
            "{}\nTry {} for more information",
            stylize("Invalid arguments", "red"),
            stylize("br --help", "yellow+italic")
        )
    } else {
        Ok(Action::Checkout)
    }
}
