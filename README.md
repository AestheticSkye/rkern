# An x86-64 kernel written purely in rust
## Compiling
Compilation requires the crate bootimage to be installed `cargo install bootimage`, as well as rust nightly.
Running requires QEMU to be in path, which can be installed on macOS with `brew install qemu` (TODO: add other platform installations)
Then the emulator can just be ran using `cargo run`
## Features
The kernel currently has paging, dynamic allocation, vga display, and interrupt implemented. See below todo list for unimplemented features.
The kernel also contains a simple virtual shell, which currently contains the commands `echo` `clear` & `history`
### Todo
- async/await
- file system
- seperation of userspace
- file loading
## Credit
Most of the code in this repo was adapted from [this](https://os.phil-opp.com) tutorial by Philipp Oppermann.
