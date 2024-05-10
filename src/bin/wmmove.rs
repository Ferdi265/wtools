#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;
use wlib::shapes;

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
    let disp = wlib::Display::open()?;
    let p = shapes::Point::new(x, y);
    match wid {
        Some(w) => {
            let win = disp.window(w)?;
            win.warp_pointer(p)?;
        },
        None => {
            match mode {
                Mode::Relative => disp.warp_pointer_relative(p)?,
                Mode::Absolute => disp.warp_pointer_absolute(p)?
            }
        }
    }
    Ok(())
}
