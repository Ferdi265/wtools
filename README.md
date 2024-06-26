# wtools - Coreutils for Xorg, implemented in Rust

This project aims to implement simple tools for window management in Xorg.
wtools is inspired by the [wmutils](https://github.com/wmutils/core) project.

This project uses my own library [wlib](https://github.com/Ferdi265/wlib) to
communicate with Xorg.

## Maintenance Note

This project is unmaintained. I built this in 2016, and didn't touch it since,
except for making it build cleanly with a modern Rust edition in 2024.

## Building

This project is inteded to be build with Cargo, the Rust package manager.

This project compiles with stable Rust 1.78.0.

## Documentation

This project is currently undocumented. This might change, but for now the code
should suffice as documentation as most files are very small and readable.

## Contributing

Contributions are welcome, although I might not accept all pull requests,
simply as a matter of taste. Don't be afraid to create a fork.

## License

This project is licensed under the GNU GPLv3 and later licenses. The GNU GPLv3 is
provided in the LICENSE file.

## Current tools

All tools have a `--help` switch that gives more info about their usage. Long
options are available, but omitted here for brevity.

### wborder

`wborder [-c color] [-s size] wid`

Change a window's border color and width.

### wdestroy

`wdestroy [-k] wid`

Destroys a window or kills the controlling client.

### wfocus

`wfocus wid`

Focus a window.

### wignore

`wignore [-rs] wid`

Set or reset `override_redirect`.

### winfo

`winfo [-xywhbimo] wid`

Gives information about a window.

### wlist

`wlist [-uoa] [wid]`

Lists the children of wid, defaults to the root window.

### wmap

`wmap [-mu] wid`

Maps or unmaps the window.

### wminfo

`wminfo [-xy] [wid]`

Mouse pointer coordinates relative to win, defaults to root window.

### wmmove

`wmmove [-ra] [wid]`

Moves mouse pointer relatively, absolutely, or offset to the origin of wid.

### wmove

`wmove [-ra] x y wid`

Move a window relatively or absolutely.

### wmovecenter

`wmovecenter [-ra -m] x y wid`

Moves a window relatively or absolutely. Moves window centered in absolute mode
and has an option to also move the mouse with it.

### wresize

`wresize [-ra] x y wid`

Resize a window relatively or absolutely.

### wstack

`wstack [-rao] wid`

Change stacking order of windows.

### wwindow

`wwindow [-rf]`

Returns the window id of special windows.

Currently supports the root window and the focused window. Prints an error if
no window is focused in focus mode rather than printing the reserved resource
ids for None and PointerRoot.
