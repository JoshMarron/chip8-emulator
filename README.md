# CHIP-8 Emulator

This project is a small emulator I built to get more comfortable with Rust. I chose to emulate the CHIP-8 Virtual Machine because of its very simple instruction set and way of working. The emulator is very simple and primitive as it is more of a training project for future work.

## Installation

Clone the repository and run cargo build. You need to install SDL2. On Linux this should be as simple as installing libsdl2-dev from a package manager. On Windows, download the development libraries from the SDL2 website.

## Running

Use cargo run \<NAME>.rom to run the emulator for a particular rom. Use cargo run -- --help to get a list of other arguments that can be passed to the program.