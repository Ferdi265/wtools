#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

#[derive(Copy, Clone)]
enum Mode {
    Relative,
    Absolute
}

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "move window",
        flag mode: Mode = Mode::Relative,
            (&["-r", "--relative"], Mode::Relative, "move relatively (default)"),
            (&["-a", "--absolute"], Mode::Absolute, "move absolutely"),
        arg x: i32,
            ("x", "x coordinate"),
        arg y: i32,
            ("y", "y coordinate"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli_util::handle_error(&name, 1, run(mode, x, y, wid));
}

fn run(mode: Mode, x: i32, y: i32, wid: window::ID) -> Result<(), &'static str> {
    let disp = wlib::Display::open()?;
    let mut win = disp.window(wid).map_err(|_| "window does not exist")?;
    match mode {
        Mode::Relative => win.reposition_relative(x, y)?,
        Mode::Absolute => win.reposition_absolute(x, y)?
    }
    Ok(())
}
