use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate clap;
use clap::App;

mod memory;
mod cpu;
mod emustate;
mod emulator;

fn main() {
    let yaml = load_yaml!("chip8.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = emulator::Config {
        filename: matches.value_of("ROM").unwrap().to_string(),
        verbose: matches.is_present("verbose")
    };

    println!("Config: {:?}", config);
}
