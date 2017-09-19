use memory::Byte;
use memory::Word;
use emustate;

pub struct Cpu {
     v_registers : Vec<Byte>,
     i_register : Word,
     stack_pointer: Word,
     sound_timer: Byte,
     delay_time: Byte,
     program_counter: Word   
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v_registers: vec![0; 16],
            i_register: Word::new_from_full(0),
            stack_pointer: Word::new_from_full(0),
            sound_timer: 0,
            delay_time: 0,
            program_counter: Word::new_from_full(emustate::PC_START)
        }
    }

    pub fn program_counter_mut(&mut self) -> &mut Word {
        &mut self.program_counter
    }

    pub fn program_counter(&self) -> &Word {
        &self.program_counter
    }
}