use cpu::Cpu;
use memory::*;
use display::Display;
use decoder;
use decoder::Instruction;
use input::Input;
use font::FONTS;

pub const PC_START : u16 = 0x200;
pub const FONT_START : u16 = 0x0;

pub struct Chip8State {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    input: Input
}

impl Chip8State {
    pub fn new() -> Chip8State {
        let display = Display::new();
        let input = Input::new(display.get_context());
        Chip8State {
            cpu: Cpu::new(),
            memory: Memory::new(4096),
            display,
            input
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

    pub fn load_font(&mut self) {
        let mut address = Word::new_from_full(FONT_START);
        for byte in FONTS.iter() {
            self.memory.write(&address, *byte);
            address += 1;
        }

        self.memory.print_mem_section(FONT_START, 0x81);
    }

    pub fn run_next_cycle(&mut self) -> Result<(), &'static str> {
        let opcode = self.fetch_instruction();
        let instruction = decoder::decode(opcode);
        
        if let Instruction::Unknown(op) = instruction {
            error!("Fatal: {} is unknown opcode", op);
            panic!();
        } else {
            debug!("{:04X} -- {:?}", self.cpu.program_counter().full(), instruction)
        }

        let keys = self.input.poll()?;

        self.cpu.run_instruction(instruction, &mut self.memory, &mut self.display, &keys);

        if self.display.vram_changed() {
            self.display.refresh_display();
        }

        debug!("{:?}", self.cpu);

        Ok(())
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