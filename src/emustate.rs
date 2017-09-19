use cpu::Cpu;
use memory::*;

pub const PC_START : u16 = 0x200;

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
        let mut address = Word::new_from_full(PC_START);
        for byte in program {
            self.memory.write(&address, byte);
            address += 1;
        }

        self.memory.print_mem_section(PC_START, 0x300);
    }

    pub fn run_next_cycle(&self) {
        let instruction = self.fetch_instruction();
        
    }

    pub fn fetch_instruction(&mut self) -> Word {
        let mut counter = self.cpu.program_counter();

        let high_byte = self.memory.read(&counter);
        counter += 1;
        let low_byte = self.memory.read(&counter);
        counter +=1;

        info!("Program counter: {:?}", counter);
        info!("Instruction fetched: {:02x}{:02x}", high_byte, low_byte);
        Word::new_from_bytes(high_byte, low_byte)
    }  
}