#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

#[derive(Copy, Clone)]
enum Mode {
    Relative,
    Absolute,
}

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "move mouse pointer",
        flag mode: Mode = Mode::Relative,
            (&["-r", "--relative"], Mode::Relative, "move relatively (default)"),
            (&["-a", "--absolute"], Mode::Absolute, "move absolutely"),
        arg x: i32,
            ("x", "x coordinate"),
        arg y: i32,
            ("y", "y coordinate"),
        optarg wid: window::ID,
            ("wid", "move pointer absolutely relative to the origin of this window")
    }
    
    cli_util::handle_error(&name, 1, run(mode, x, y, wid));
}

fn run(mode: Mode, x: i32, y: i32, wid: Option<window::ID>) -> Result<(), &'static str> {
    let disp = try!(wlib::Display::open());
    match wid {
        Some(w) => {
            let win = try!(disp.window(w));
            try!(win.warp_pointer(x, y));
        },
        None => {
            match mode {
                Mode::Relative => try!(disp.warp_pointer_relative(x, y)),
                Mode::Absolute => try!(disp.warp_pointer_absolute(x, y))
            }
        }
    }
    Ok(())
}
