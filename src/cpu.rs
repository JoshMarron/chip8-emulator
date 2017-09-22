extern crate rand;
use self::rand::Rng;
use std::num::Wrapping;

use memory::Byte;
use memory::Word;
use memory::Memory;

use display::Display;
use display::Sprite;

use emustate;
use decoder::Instruction;
use util;

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

    pub fn run_instruction(&mut self, instruction: Instruction, memory: &mut Memory, display: &mut Display, keys: &[bool]) {
        if self.delay_time > 0 {
            self.delay_time -= 1;
        }
        match instruction {
            Instruction::CLS => {
                display.clear_screen();
            }
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
                } else {
                    self.set_reg(0xF, 0);
                }
                let new_val = Wrapping(self.get_reg(reg_val.register)) + Wrapping(reg_val.value);
                self.set_reg(reg_val.register, new_val.0);
            },
            Instruction::LDR(registers) => {
                let value = self.get_reg(registers.second_reg);
                self.set_reg(registers.first_reg, value);
            },
            Instruction::ORR(registers) => {
                let value = self.get_reg(registers.second_reg);
                let current = self.get_mut_reg(registers.first_reg);
                *current |= value;
            },
            Instruction::ANDR(registers) => {
                let value = self.get_reg(registers.second_reg);
                let current = self.get_mut_reg(registers.first_reg);
                *current &= value;
            },
            Instruction::XORR(registers) => {
                let value = self.get_reg(registers.second_reg);
                let current = self.get_mut_reg(registers.first_reg);
                *current ^= value;
            },
            Instruction::ADDR(registers) => {
                let new_val = self.get_reg(registers.first_reg) as u16 + self.get_reg(registers.second_reg) as u16;
                if new_val > 255 {
                    self.set_reg(0xF, 1);
                } else {
                    self.set_reg(0xF, 0);
                }
                let new_val = Wrapping(self.get_reg(registers.first_reg)) + Wrapping(self.get_reg(registers.second_reg));
                self.set_reg(registers.first_reg, new_val.0);
            },
            Instruction::SUBR(registers) => {
                let reg1 = self.get_reg(registers.first_reg);
                let reg2 = self.get_reg(registers.second_reg);

                if reg1 > reg2 {
                    self.set_reg(0xF, 1);
                } else {
                    self.set_reg(0xF, 0);
                }

                self.set_reg(registers.first_reg, (Wrapping(reg1) - Wrapping(reg2)).0);
            },
            Instruction::SHR(reg) => {
                let current = self.get_reg(reg);
                self.set_reg(0xF, current & 0b00000001);
                self.set_reg(reg, current >> 1);
            },
            Instruction::SUBNR(registers) => {
                let reg1 = self.get_reg(registers.first_reg);
                let reg2 = self.get_reg(registers.second_reg);

                if reg2 > reg1 {
                    self.set_reg(0xF, 1);
                } else {
                    self.set_reg(0xF, 0);
                }
                self.set_reg(registers.first_reg, (Wrapping(reg2) - Wrapping(reg1)).0)
            },
            Instruction::SHL(reg) => {
                let current = self.get_reg(reg);
                self.set_reg(0xF, current >> 7);
                self.set_reg(reg, current << 1);
            },
            Instruction::SNER(registers) => {
                if self.get_reg(registers.first_reg) != self.get_reg(registers.second_reg) {
                    self.program_counter += 2;
                }
            },
            Instruction::LDI(address) => {
                self.i_register = address;
            },
            Instruction::JUMPV0(address) => {
                self.program_counter = address + self.get_reg(0) as u16;
            },
            Instruction::RND(reg_val) => {
                let mut rng = rand::thread_rng();
                let rand = rng.gen_range(0, 256) as Byte;
                self.set_reg(reg_val.register, rand & reg_val.value);
            },
            Instruction::DRW(reg_nibble) => {
                let x = self.get_reg(reg_nibble.first_reg);
                let y = self.get_reg(reg_nibble.second_reg);
                let sprite = Sprite::new(memory.read_slice(&self.i_register, reg_nibble.nibble));
                self.set_reg(0xF, display.draw_sprite(x, y, sprite));
            },
            Instruction::SKP(reg) => {
                let keycode = self.get_reg(reg);
                if keys[keycode as usize] {
                    self.program_counter += 2;
                }
            },
            Instruction::SKNP(reg) => {
                let keycode = self.get_reg(reg);
                if !keys[keycode as usize] {
                    self.program_counter += 2;
                }
            },
            Instruction::LDVDT(reg) => {
                let delay = self.delay_time;
                self.set_reg(reg, delay);
            },
            Instruction::LDK(reg) => {
                for (i, key) in keys.iter().enumerate() {
                    if *key {
                        self.set_reg(reg, i as u8);
                        return;
                    }
                }
                self.program_counter -= 2;
            },
            Instruction::LDDTV(reg) => {
                self.delay_time = self.get_reg(reg);
            },
            Instruction::LDSTV(reg) => {
                self.sound_timer = self.get_reg(reg);
            },
            Instruction::ADDI(reg) => {
                let i_val = self.i_register.clone();
                self.i_register = i_val + self.get_reg(reg) as u16;
            },
            Instruction::LDFONT(reg) => {
                let font_code = self.get_reg(reg);
                self.i_register = memory.get_font(font_code);
            },
            Instruction::LDBCD(reg) => {
                let val = self.get_reg(reg);
                let mut digits : Vec<Byte> = Vec::with_capacity(3);
                util::get_digits(val as u16, &mut digits);
                
                let mut address = self.i_register.clone();

                for digit in digits {
                    memory.write(&address, digit);
                    address += 1;
                }

                memory.print_mem_section(address.full() - 3, address.full());
            },
            Instruction::STARR(reg) => {
                let mut address = self.i_register.clone();
                for n in 0..reg + 1 {
                    memory.write(&address, self.get_reg(n));
                    address += 1;
                }

                memory.print_mem_section(address.full() - (reg + 1) as u16, address.full());
            },
            Instruction::LDARR(reg) => {
                let mut address = self.i_register.clone();
                for n in 0..reg + 1 {
                    self.set_reg(n, memory.read(&address));
                    address += 1;
                }
            },
            Instruction::Unknown(opcode) => {
                error!("Fatal: unknown opcode: {}", opcode);
                panic!();
            }
        }
    }
}