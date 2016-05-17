# wtools-simple - Coreutils for Xorg, implemented in Rust

This project aims to implement simple tools for window management in Xorg.
wtools is inspired by the [wmutils](https://github.com/wmutils/core) project.

This project uses Xlib (Cargo crate [x11](https://crates.io/crates/x11)) to
communicate with Xorg, as XCB was very hard to abstract to my needs.

## Building

This project is inteded to be build with Cargo, the Rust package manager.

This project specifically requires (as of 2016.05.08) a nightly build of rustc
to compile, as it uses some macro code (`parse_args!` from
[cli-util](https://github.com/Ferdi265/cli-util) inexplicably doesn't work on
release rustc) that doesn't work with release rustc.

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
