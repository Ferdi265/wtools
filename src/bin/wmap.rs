#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

#[derive(Copy, Clone)]
enum Mode {
    Map,
    Unmap
}

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "map or unmap window",
        flag mode: Mode = Mode::Map,
            (&["-m", "--map"], Mode::Map, "map window (default)"),
            (&["-u", "--unmap"], Mode::Unmap, "unmap window"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli_util::handle_error(&name, 1, run(mode, wid));
}

fn run(mode: Mode, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wlib::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    match mode {
        Mode::Map => try!(win.map()),
        Mode::Unmap => try!(win.unmap())
    }
    Ok(())
}
