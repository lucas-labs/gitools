use {
    lool::cli::stylize::stylize,
    spinners::{Spinner, Spinners},
};

pub fn make_spinner(msg: String) -> Spinner {
    Spinner::new(Spinners::Dots7, msg)
}

pub fn stop_spinner(sp: &mut Spinner, msg: String, color: &str, symbol: &str) {
    sp.stop_and_persist(&stylize(symbol, color), stylize(msg, color));
}

pub fn stop_spinner_and_clear(sp: &mut Spinner) {
    sp.stop();
    print!("\x1b[2K\r"); // clear the spinner
}

pub struct StatusMessage {
    spinner: Spinner,
}

impl StatusMessage {
    pub fn start(msg: &str) -> Self {
        let spinner = make_spinner(msg.to_owned());
        StatusMessage { spinner }
    }

    pub fn stop_with(&mut self, msg: &str, status: bool) {
        let col = if status { "green" } else { "red" };
        let symbol = if status { "✔" } else { "✖" };

        stop_spinner(&mut self.spinner, msg.to_owned(), col, symbol);
    }

    pub fn stop_with_or_kill(&mut self, msg: &str, status: bool, kill: bool) {
        if kill {
            stop_spinner_and_clear(&mut self.spinner);
        } else {
            self.stop_with(msg, status);
        }
    }
}
