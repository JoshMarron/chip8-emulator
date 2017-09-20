use std::ops;
use std::fmt;

#[derive(Clone)]
pub struct Word {
    full: u16,
    high: Byte,
    low: Byte
}

pub type Byte = u8;

impl Word {
    pub fn new_from_full(full: u16) -> Word {
        Word {
            full,
            high: ((full & 0xFF00) >> 8) as Byte,
            low: (full & 0x00FF) as Byte
        }
    }

    pub fn new_from_bytes(high: Byte, low: Byte) -> Word {
        Word {
            full: ((high as u16) << 8) + low as u16,
            high: high,
            low: low
        }
    }

    pub fn full(&self) -> u16 {
        self.full.clone()
    }

    pub fn high(&self) -> u8 {
        self.high.clone()
    }

    pub fn low(&self) -> u8 {
        self.low.clone()
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}", self.full)
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "full: {:04x} high: {:02x} low {:02x}", self.full, self.high, self.low)
    }
}

impl<'a> ops::Add<u16> for &'a mut Word {
    type Output = Word;

    fn add(self, other: u16) -> Word {
        Word::new_from_full(self.full + other)
    }
}

impl ops::Add<u16> for Word {
    type Output = Word;

    fn add(self, other: u16) -> Word {
        Word::new_from_full(self.full + other)
    }
}

impl ops::SubAssign<u16> for Word {
    fn sub_assign(&mut self, val: u16) {
        self.full = self.full - val;
        self.high = (self.full >> 8) as Byte;
        self.low = (self.full & 0x00FF) as Byte;
    }
}

impl ops::AddAssign<u16> for Word {
    fn add_assign(&mut self, val: u16) {
        self.full = self.full + val;
        self.high = (self.full >> 8) as Byte;
        self.low = (self.full & 0x00FF) as Byte;
    }
}

impl<'a> ops::AddAssign<u16> for &'a mut Word {
    fn add_assign(&mut self, val: u16) {
        self.full = self.full + val;
        self.high = (self.full >> 8) as Byte;
        self.low = (self.full & 0x00FF) as Byte;
    }
}

#[derive(Debug)]
pub struct Memory {
    memory: Vec<Byte>,
    stack: Vec<Word>,
    memory_size: usize
}

impl Memory {
    pub fn new(memory_size: usize) -> Memory {
        Memory {
            memory: vec![0; memory_size],
            stack: Vec::with_capacity(16),
            memory_size
        }
    }

    pub fn read(&self, address: &Word) -> Byte {
        if address.full >= self.memory_size as u16 {
            panic!(format!("Fatal: tried to read out of memory range: {:04X}", address.full));
        }
        self.memory[address.full as usize]
    }

    pub fn write(&mut self, address: &Word, value: Byte) {
        if address.full >= self.memory_size as u16 {
            panic!(format!("Fatal: tried to write out of memory range: {:04X}", address.full));
        }
        self.memory[address.full as usize] = value;
    }

    pub fn pop_stack(&mut self) -> Option<Word> {
        self.stack.pop()
    }

    pub fn push_stack(&mut self, value: Word) {
        self.stack.push(value);
        trace!("{:?}", self.stack);
    }

    pub fn print_mem_section(&self, start: u16, end: u16) {
        for (address, data) in self.memory.iter().enumerate() {
            if address as u16 >= start && address as u16 <= end {
                trace!("{:04X} : {:02x}", address, data);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use memory;
    use std::panic;

    #[test]
    fn test_read_out_of_range() {
        let memory = memory::Memory::new(16);
        let address = memory::Word::new_from_full(18);

        let result = panic::catch_unwind(|| {
            memory.read(&address);
        });

        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_write_out_of_range() {
        let mut memory = memory::Memory::new(16);
        let address = memory::Word::new_from_full(27);

        memory.write(&address, 14);
    }

    #[test]
    fn test_write_read() {
        let mut memory = memory::Memory::new(16);
        let address = memory::Word::new_from_full(12);

        memory.write(&address, 128);

        assert_eq!(memory.read(&address), 128);
    }
}