#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "ignore window",
        flag mode: bool = true,
            (&["-r", "--reset"], false, "sets override_redirect to false"),
            (&["-s", "--set"], true, "sets override_redirect to true (default)"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli_util::handle_error(&name, 1, run(mode, wid));
}

fn run(mode: bool, wid: window::ID) -> Result<(), &'static str> {
    let disp = wlib::Display::open()?;
    let mut win = disp.window(wid).map_err(|_| "window does not exist")?;
    win.ignore(mode)?;
    Ok(())
}
