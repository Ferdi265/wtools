#[macro_use]
extern crate cli_util;
extern crate wlib;

use std::env;
use wlib::window;
use wlib::shapes;

#[derive(Copy, Clone)]
enum Mode {
    Relative,
    Absolute
}

fn main() {
    let name = cli_util::name(&mut env::args());

    parse_args!{
        description: "move window centered",
        flag mode: Mode = Mode::Relative,
            (&["-r", "--relative"], Mode::Relative, "move relatively (default)"),
            (&["-a", "--absolute"], Mode::Absolute, "move absolutely"),
        flag mouse: bool = false,
            (&["-m", "--mouse"], true, "move the mouse with the window"),
        arg x: i32,
            ("x", "x coordinate"),
        arg y: i32,
            ("y", "y coordinate"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli_util::handle_error(&name, 1, run(mode, mouse, x, y, wid));
}

fn run(mode: Mode, mouse: bool, x: i32, y: i32, wid: window::ID) -> Result<(), &'static str> {
    let disp = wlib::Display::open()?;
    let mut win = disp.window(wid).map_err(|_| "window does not exist")?;
    match mode {
        Mode::Relative => {
            if mouse {
                disp.warp_pointer_relative(shapes::Point::new(x, y))?;
            }
            win.reposition_relative(x, y)?;
        },
        Mode::Absolute => {
            let r = win.frame().r;
            let c = r.corner(shapes::Corner::CENTER);
            let x = x - c.x;
            let y = y - c.y;
            win.reposition_absolute(x, y)?;
            if mouse {
                // window center has moved, recalculate
                let r = win.frame().r;
                let c = r.corner(shapes::Corner::CENTER);
                win.warp_pointer(c)?;
            }
        }
    }
    Ok(())
}
