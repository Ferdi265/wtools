#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;

#[derive(Copy, Clone)]
enum Mode {
    Root,
    Focus
}

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "returns special windows",
        flag mode: Mode = Mode::Root,
            (&["-r", "--root"], Mode::Root, "root window (default)"),
            (&["-f", "--focus"], Mode::Focus, "focused window")
    }

    cli_util::handle_error(&name, 1, run(mode));
}

fn run(mode: Mode) -> Result<(), &'static str> {
    let disp = try!(wlib::Display::open());
    let win = match mode {
        Mode::Root => {
            let scrn = try!(disp.screen());
            try!(scrn.root())
        },
        Mode::Focus => {
            let r = disp.focus()
                .and_then(|wo| wo.ok_or("no window focused"));
            try!(r)
        }
    };
    println!("{}", win.id());
    Ok(())
}
