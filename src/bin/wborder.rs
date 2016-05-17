#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "change window border",
        opt color: wlib::Color,
            (&["-c", "--color"], "border color"),
        opt size: i32,
            (&["-s", "--size"], "border size"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli_util::handle_error(&name, 1, run(color, size, wid));
}

fn run(color: Option<wlib::Color>, size: Option<i32>, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wlib::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    let mut c = window::Changes::new();
    if let Some(color) = color {
        c.border_color(color);
    }
    if let Some(size) = size {
        c.border_width(size);
    }
    try!(win.change(&c));
    Ok(())
}
