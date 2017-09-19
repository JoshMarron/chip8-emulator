use memory::Word;
use memory::byte;

// These are in the order found at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

#[derive(Debug)]
pub enum Instruction {
    Unknown,
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
    SHR(byte),                  // Shift right
    SUBNR(RegisterRegister),
    SHL(byte),
    SNER(RegisterRegister),
    LDI(byte),
    JUMPV0(byte),
    RND(RegisterVal),
    DRW(RegisterRegisterNibble),
    SKP(byte),
    SKNP(byte),
    LDVDT(byte),                // Load DT into Vx
    LDK(byte),
    LDDTV(byte),                // Load Vx into DT
    LDSTV(byte),
    ADDI(byte),
    LDFONT(byte),
    LDBCD(byte),
    STARR(byte),
    LDARR(byte)
}

#[derive(Debug)]
pub struct RegisterVal {
    pub register: byte,
    pub value: byte
}

#[derive(Debug)]
pub struct RegisterRegister {
    pub first_reg: byte,
    pub second_reg: byte
}

#[derive(Debug)]
pub struct RegisterRegisterNibble {
    pub first_reg: byte,
    pub second_reg: byte,
    pub nibble: byte
}

pub fn decode(opcode: Word) -> Instruction {
    
    return match opcode.high() {
        0x00...0x0F => {
            decode_flow(&opcode)
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
            info!("{:02x} and {:02X}", first_reg, second_reg);
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
            decode_operations(&opcode)
        }
        _ => Instruction::Unknown
    }
}

pub fn decode_flow(opcode: &Word) -> Instruction {
    return match opcode.full() {
        0x00E0 => {
            Instruction::CLS
        },
        0x00EE => {
            Instruction::RET
        },
        _ => Instruction::Unknown
    }
}

pub fn decode_operations(opcode: &Word) -> Instruction {
    let (first_reg, second_reg) = get_both_registers(opcode);

    return match opcode.low() & 0x0F {
        0x0 => {
            info!("LDR opcode: {}", opcode);
            Instruction::LDR(RegisterRegister {first_reg, second_reg})
        },
        0x1 => {
            info!("ORR opcode: {}", opcode);
            Instruction::ORR(RegisterRegister {first_reg, second_reg})
        },
        0x2 => {
            info!("ANDR opcode: {}", opcode);
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
        _ => Instruction::Unknown
    }
}

pub fn get_register(opcode: &Word) -> byte {
    opcode.high() & 0x0F
}

pub fn get_value(opcode: &Word) -> byte {
    opcode.low()
}

pub fn get_both_registers(opcode: &Word) -> (byte, byte) {
    (opcode.high() & 0x0F, opcode.low() >> 4)
}

pub fn get_address(opcode: &Word) -> Word {
    Word::new_from_full(opcode.full() & 0x0FFF)
}