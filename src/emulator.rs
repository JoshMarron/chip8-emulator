use emustate::Chip8State;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub verbose: bool
}

pub fn run(state: Chip8State) -> Result<(), Box<Error>> {
    Ok(())
}