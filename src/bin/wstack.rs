#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "change window's position on the stack",
        flag mode: window::StackMode = window::StackMode::Above,
            (&["-a", "--above"], window::StackMode::Above, "move to top of stack (default)"),
            (&["-b", "--below"], window::StackMode::Below, "move to bottom of stack"),
            (&["-o", "--opposite"], window::StackMode::Opposite, "invert window's position on stack"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli_util::handle_error(&name, 1, run(mode, wid));
}

fn run(mode: window::StackMode, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wlib::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    try!(win.restack(mode));
    Ok(())
}
