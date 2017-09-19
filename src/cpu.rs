use memory::byte;
use memory::Word;
use emustate;

pub struct Cpu {
     v_registers : Vec<byte>,
     i_register : Word,
     stack_pointer: Word,
     sound_timer: byte,
     delay_time: byte,
     program_counter: Word   
}

impl<'a> Cpu {
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

    pub fn program_counter(&mut self) -> &mut Word {
        &mut self.program_counter
    }
}