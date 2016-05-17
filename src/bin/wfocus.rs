#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "focus window",
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli_util::handle_error(&name, 1, run(wid));
}

fn run(wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wlib::Display::open());
    let win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    try!(win.focus());
    Ok(())
}
