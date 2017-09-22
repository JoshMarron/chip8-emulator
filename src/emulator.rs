use emustate::Chip8State;
use memory::Byte;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::thread;
use std::time;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub verbose: bool,
    pub debug: bool
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut rom_file = File::open(config.filename)?;
    let mut bytes : Vec<Byte> = Vec::new(); 
    rom_file.read_to_end(&mut bytes)?;

    info!("Bytes size: {}", bytes.len());

    let mut init_state = Chip8State::new();
    init_state.load_instructions(bytes);
    init_state.load_font();

    loop_emulation(init_state, config.debug)?;

    Ok(())
}

pub fn loop_emulation(mut state: Chip8State, debug: bool) -> Result<(), Box<Error>> {
    let stdin = io::stdin();

    if debug {
        loop {
            let mut input = String::new();
            stdin.read_line(&mut input)?;
            if input.trim() == "n" {
                state.run_next_cycle()?;
            } else if input.trim() == "exit" {
                break;
            } else {
                error!("Invalid command: {}", input);
                continue;
            }
        }
    } else {
        while let Ok(()) = state.run_next_cycle() {
            thread::sleep(time::Duration::from_millis(2));
        }
    }

    Ok(())
}