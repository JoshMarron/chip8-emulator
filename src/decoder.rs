use memory::Word;
use memory::byte;

// These are in the order found at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

#[derive(Debug)]
pub enum Instruction {
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
        0x60...0x6F => {
            info!("{}", opcode);
            let register = get_register(&opcode);
            let value = get_value(&opcode);
            info!("{} into {:X}", value, register);
            Instruction::LD(RegisterVal {register, value})
        },
        _ => Instruction::CLS
    }
}

pub fn get_register(opcode: &Word) -> byte {
    opcode.high() & 0x0F
}

pub fn get_value(opcode: &Word) -> byte {
    opcode.low()
}