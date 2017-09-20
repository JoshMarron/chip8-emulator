use memory::Byte;
use memory::Word;
use memory::Memory;
use emustate;
use decoder::Instruction;

use std::mem;

#[derive(Debug)]
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

    pub fn get_reg(&self, register: Byte) -> Byte {
        assert!((register as usize) < self.v_registers.len());

        self.v_registers[register as usize]
    }

    pub fn set_reg(&mut self, register: Byte, value: Byte) {
        assert!((register as usize) < self.v_registers.len());

        self.v_registers[register as usize] = value;
    }

    pub fn get_mut_reg(&mut self, register: Byte) -> &mut u8 {
        if let Some(current) = self.v_registers.get_mut(register as usize) {
            current 
        } else {
            error!("Fatal: tried to access non-existent register");
            panic!();
        }
    }

    pub fn run_instruction(&mut self, instruction: Instruction, memory: &mut Memory) {
        match instruction {
            Instruction::RET => {
                if let Some(value) = memory.pop_stack() {
                    self.program_counter = value;
                    self.stack_pointer -= 1;
                } else {
                    error!("Fatal: attempted to pop empty stack");
                    panic!();
                }
            },
            Instruction::JUMP(address) => {
                self.program_counter = address;
            },
            Instruction::CALL(address) => {
                memory.push_stack(mem::replace(&mut self.program_counter, address));
                self.stack_pointer += 1;
            },
            Instruction::SE(reg_val) => {
                if self.get_reg(reg_val.register) == reg_val.value {
                    self.program_counter += 2;
                }
            },
            Instruction::SNE(reg_val) => {
                if self.get_reg(reg_val.register) != reg_val.value {
                    self.program_counter += 2;
                }
            },
            Instruction::SER(registers) => {
                if self.get_reg(registers.first_reg) == self.get_reg(registers.second_reg) {
                    self.program_counter += 2;
                }
            }
            Instruction::LD(reg_val) => {
                self.set_reg(reg_val.register, reg_val.value);
            },
            Instruction::ADD(reg_val) => {
                let current = self.get_reg(reg_val.register);
                let new_val = current as u16 + reg_val.value as u16;
                if new_val > 255 {
                    self.set_reg(0xF, 1);
                }
                self.set_reg(reg_val.register, new_val as Byte);
            },
            Instruction::LDR(registers) => {
                let value = self.get_reg(registers.second_reg);
                self.set_reg(registers.first_reg, value);
            },
            Instruction::ORR(registers) => {
                let value = self.get_reg(registers.second_reg);
                let current = self.get_mut_reg(registers.first_reg);
                *current |= value;
            }
            _ => error!("Unimplemented: {:?}", instruction)
        }
    }
}