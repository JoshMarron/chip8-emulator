pub struct Word {
    full: u16,
    high: byte,
    low: byte
}

pub type byte = u8;

impl Word {
    pub fn new_from_full(full: u16) -> Word {
        Word {
            full,
            high: ((full & 0xFF00) >> 8) as byte,
            low: (full & 0x00FF) as byte
        }
    }

    pub fn new_from_bytes(high: byte, low: byte) -> Word {
        Word {
            full: ((high as u16) << 8) + low as u16,
            high: high,
            low: low
        }
    }
}

pub struct Memory {
    memory: Vec<byte>,
    memory_size: usize
}

impl Memory {
    pub fn new(memory_size: usize) -> Memory {
        Memory {
            memory: vec![0; memory_size],
            memory_size
        }
    }

    pub fn read(&self, address: Word) -> byte {
        if address.full >= self.memory_size as u16 {
            panic!(format!("Fatal: tried to read out of memory range: {:04X}", address.full));
        }
        self.memory[address.full as usize]
    }

    pub fn write(&mut self, address: Word, value: byte) {
        if address.full >= self.memory_size as u16 {
            panic!(format!("Fatal: tried to write out of memory range: {:04X}", address.full));
        }
        self.memory[address.full as usize] = value;
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
            memory.read(address);
        });

        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_write_out_of_range() {
        let mut memory = memory::Memory::new(16);
        let address = memory::Word::new_from_full(27);

        memory.write(address, 14);
    }
}