use cpu::Cpu;
use memory::*;

pub struct Chip8State {
    cpu: Cpu,
    memory: Memory
}

impl Chip8State {
    pub fn new() -> Chip8State {
        Chip8State {
            cpu: Cpu::new(),
            memory: Memory::new(4096)
        }
    }

    pub fn load_instructions(&mut self, program: Vec<byte>) {
        let mut address = Word::new_from_full(0x200);
        for byte in program {
            self.memory.write(&address, byte);
            address += 1;
        }

        //self.memory.print_mem_section(0x200, 0x400);
    }
}