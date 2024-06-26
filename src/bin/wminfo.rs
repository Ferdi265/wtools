#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;

#[derive(Copy, Clone)]
enum Attr {
    X,
    Y,
}

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "mouse information",
        list attrs: Attr,
            (&["-x"], Attr::X, "x coordinate"),
            (&["-y"], Attr::Y, "y coordinate"),
        optarg wid: window::ID,
            ("wid", "window id")
    }
    
    cli_util::handle_error(&name, 1, run(attrs, wid));
}

fn run(attrs: Vec<Attr>, wid: Option<window::ID>) -> Result<(), &'static str> {
    let disp = wlib::Display::open()?;
    let pos = match wid {
        Some(w) => {
            let win = disp.window(w)?;
            win.pointer()?
        },
        None => disp.pointer()?
    };
    for a in attrs {
        match a {
            Attr::X => print!("{} ", pos.x),
            Attr::Y => print!("{} ", pos.y),
        };
    }
    println!("");
    Ok(())
}
