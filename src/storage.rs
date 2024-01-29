use crate::{instruction::IntReg, register::Registers};

pub struct Memory {
    mem: Vec<u16>
}

impl Memory {
    pub fn new() -> Self {
        Self { mem: vec![0; 2_usize.pow(15)] }
    }

    pub fn read(&self, a: u16) -> u16 {
        self.mem[a as usize]
    }

    pub fn write(&mut self, a: u16, val: u16) {
        self.mem[a as usize] = val;
    }
}

pub struct Storage {
    pub regs: Registers,
    pub stack: Vec<u16>,
    pub mem: Memory,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            stack: Vec::new(),
            mem: Memory::new(),
        }
    }
}
