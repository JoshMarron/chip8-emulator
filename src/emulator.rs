use emustate::Chip8State;
use memory::byte;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub verbose: bool
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut rom_file = File::open(config.filename)?;
    let mut bytes : Vec<byte> = Vec::new(); 
    rom_file.read_to_end(&mut bytes)?;

    info!("Bytes size: {}", bytes.len());

    let mut init_state = Chip8State::new();
    init_state.load_instructions(bytes);

    loop_emulation(init_state);

    Ok(())
}

pub fn loop_emulation(mut state: Chip8State) {
    loop {
        state.run_next_cycle();
        break;
    }
}