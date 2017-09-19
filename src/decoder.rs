use memory::Word;
use memory::Byte;

// These are in the order found at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

#[derive(Debug)]
pub enum Instruction {
    Unknown(Word),          // Word is opcode
    CLS,
    RET,
    JUMP(Word),
    CALL(Word),
    SE(RegisterVal),
    SNE(RegisterVal),
    SER(RegisterRegister),
    LD(RegisterVal),
    ADD(RegisterVal),
    LDR(RegisterRegister),
    ORR(RegisterRegister),
    ANDR(RegisterRegister),
    XORR(RegisterRegister),
    ADDR(RegisterRegister),
    SUBR(RegisterRegister),
    SHR(Byte),                  // Shift right
    SUBNR(RegisterRegister),
    SHL(Byte),
    SNER(RegisterRegister),
    LDI(Word),
    JUMPV0(Word),
    RND(RegisterVal),
    DRW(RegisterRegisterNibble),
    SKP(Byte),
    SKNP(Byte),
    LDVDT(Byte),                // Load DT into Vx
    LDK(Byte),
    LDDTV(Byte),                // Load Vx into DT
    LDSTV(Byte),
    ADDI(Byte),
    LDFONT(Byte),
    LDBCD(Byte),
    STARR(Byte),
    LDARR(Byte)
}

#[derive(Debug)]
pub struct RegisterVal {
    pub register: Byte,
    pub value: Byte
}

#[derive(Debug)]
pub struct RegisterRegister {
    pub first_reg: Byte,
    pub second_reg: Byte
}

#[derive(Debug)]
pub struct RegisterRegisterNibble {
    pub first_reg: Byte,
    pub second_reg: Byte,
    pub nibble: Byte
}

pub fn decode(opcode: Word) -> Instruction {
    
    return match opcode.high() {
        0x00...0x0F => {
            decode_flow(opcode)
        },
        0x10...0x1F => {
            let address = get_address(&opcode);
            Instruction::JUMP(address)
        },
        0x20...0x2F => {
            let address = get_address(&opcode);
            Instruction::CALL(address)
        },
        0x30...0x3F => {
            let register = get_register(&opcode);
            let value = get_value(&opcode);
            Instruction::SE(RegisterVal {register, value})
        },
        0x40...0x4F => {
            let register = get_register(&opcode);
            let value = get_value(&opcode);
            Instruction::SNE(RegisterVal {register, value})
        },
        0x50...0x5F => {
            let (first_reg, second_reg) = get_both_registers(&opcode);
            Instruction::SER(RegisterRegister {first_reg, second_reg})
        }
        0x60...0x6F => {
            let register = get_register(&opcode);
            let value = get_value(&opcode);
            Instruction::LD(RegisterVal {register, value})
        },
        0x70...0x7F => {
            let register = get_register(&opcode);
            let value = get_value(&opcode);
            Instruction::ADD(RegisterVal {register, value})
        },
        0x80...0x8F => {
            decode_operations(opcode)
        },
        0x90...0x9F => {
            let (first_reg, second_reg) = get_both_registers(&opcode);
            Instruction::SNER(RegisterRegister {first_reg, second_reg})
        },
        0xA0...0xAF => {
            let address = get_address(&opcode);
            Instruction::LDI(address)
        },
        0xB0...0xBF => {
            let address = get_address(&opcode);
            Instruction::JUMPV0(address)
        },
        0xC0...0xCF => {
            let register = get_register(&opcode);
            let value = get_value(&opcode);
            Instruction::RND(RegisterVal {register, value})
        },
        0xD0...0xDF => {
            let (first_reg, second_reg, nibble) = get_both_registers_and_nibble(&opcode);
            Instruction::DRW(RegisterRegisterNibble {first_reg, second_reg, nibble})
        },
        0xE0...0xEF => {
            decode_skip_keys(opcode)
        },
        0xF0...0xFF => {
            decode_register_ops(opcode)
        }
        _ => Instruction::Unknown(opcode)
    }
}

pub fn decode_flow(opcode: Word) -> Instruction {
    return match opcode.full() {
        0x00E0 => {
            Instruction::CLS
        },
        0x00EE => {
            Instruction::RET
        },
        _ => Instruction::Unknown(opcode)
    }
}

pub fn decode_operations(opcode: Word) -> Instruction {
    let (first_reg, second_reg) = get_both_registers(&opcode);

    return match opcode.low() & 0x0F {
        0x0 => {
            Instruction::LDR(RegisterRegister {first_reg, second_reg})
        },
        0x1 => {
            Instruction::ORR(RegisterRegister {first_reg, second_reg})
        },
        0x2 => {
            Instruction::ANDR(RegisterRegister {first_reg, second_reg})
        },
        0x3 => {
            Instruction::XORR(RegisterRegister {first_reg, second_reg})
        },
        0x4 => {
            Instruction::ADDR(RegisterRegister {first_reg, second_reg})
        },
        0x5 => {
            Instruction::SUBR(RegisterRegister {first_reg, second_reg})
        },
        0x6 => {
            Instruction::SHR(first_reg)
        },
        0x7 => {
            Instruction::SUBNR(RegisterRegister {first_reg, second_reg})
        },
        0xE => {
            Instruction::SHL(first_reg)
        }
        _ => Instruction::Unknown(opcode)
    }
}

pub fn decode_skip_keys(opcode: Word) -> Instruction {
    let register = get_register(&opcode);
    return match opcode.low() {
        0x9E => {
            Instruction::SKP(register)
        },
        0xA1 => {
            Instruction::SKNP(register)
        },
        _ => Instruction::Unknown(opcode)
    }
}

pub fn decode_register_ops(opcode: Word) -> Instruction {
    let register = get_register(&opcode);
    return match opcode.low() {
        0x07 => {
            Instruction::LDVDT(register)
        },
        0x0A => {
            Instruction::LDK(register)
        },
        0x15 => {
            Instruction::LDDTV(register)
        },
        0x18 => {
            Instruction::LDSTV(register)
        },
        0x1E => {
            Instruction::ADDI(register)
        },
        0x29 => {
            Instruction::LDFONT(register)
        },
        0x33 => {
            Instruction::LDBCD(register)
        },
        0x55 => {
            Instruction::STARR(register)
        },
        0x65 => {
            Instruction::LDARR(register)
        }
        _ => Instruction::Unknown(opcode)
    }   
}

pub fn get_register(opcode: &Word) -> Byte {
    opcode.high() & 0x0F
}

pub fn get_value(opcode: &Word) -> Byte {
    opcode.low()
}

pub fn get_both_registers(opcode: &Word) -> (Byte, Byte) {
    (opcode.high() & 0x0F, opcode.low() >> 4)
}

pub fn get_both_registers_and_nibble(opcode: &Word) -> (Byte, Byte, Byte) {
    (opcode.high() & 0x0F, opcode.low() >> 4, opcode.low() & 0x0F)
}

pub fn get_address(opcode: &Word) -> Word {
    Word::new_from_full(opcode.full() & 0x0FFF)
}