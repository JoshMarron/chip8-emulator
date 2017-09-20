use cpu::Cpu;
use memory::*;
use decoder;
use decoder::Instruction;

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

    pub fn load_instructions(&mut self, program: Vec<Byte>) {
        let mut address = Word::new_from_full(PC_START);
        for byte in program {
            self.memory.write(&address, byte);
            address += 1;
        }

        self.memory.print_mem_section(PC_START, 0x300);
    }

    pub fn run_next_cycle(&mut self) {
        let opcode = self.fetch_instruction();
        let instruction = decoder::decode(opcode);
        
        if let Instruction::Unknown(op) = instruction {
            error!("Fatal: {} is unknown opcode", op);
            panic!();
        } else {
            debug!("{:04X} -- {:?}", self.cpu.program_counter().full(), instruction)
        }

        self.cpu.run_instruction(instruction, &mut self.memory);

        debug!("{:?}", self.cpu);
    }

    pub fn fetch_instruction(&mut self) -> Word {
        let mut counter = self.cpu.program_counter_mut();

        let high_byte = self.memory.read(&counter);
        counter += 1;
        let low_byte = self.memory.read(&counter);
        counter +=1;

        debug!("Program counter: {:?}", counter);
        debug!("Instruction fetched: {:02x}{:02x}", high_byte, low_byte);
        Word::new_from_bytes(high_byte, low_byte)
    }  
}