use crate::{binary::load_bin, register::Registers};

// The binary we are loading contains both the instructions and data.
// In other words, it's a shared address space.
pub struct Memory {
    mem: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        let bin = load_bin();
        Self { mem: bin }
    }

    pub fn read(&self, a: u16) -> u16 {
        self.mem[a as usize]
    }

    pub fn write(&mut self, a: u16, val: u16) {
        self.mem[a as usize] = val;
    }
}

pub struct Storage {
    pub mem: Memory,
    pub regs: Registers,
    pub stack: Vec<u16>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            regs: Registers::new(),
            stack: Vec::new(),
        }
    }
}
