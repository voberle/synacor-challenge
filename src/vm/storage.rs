use std::fs;

use crate::vm::register::Registers;

fn load_bin() -> Vec<u16> {
    let bytes = fs::read("resources/challenge.bin").unwrap();
    // Converting to u16 with safe code
    bytes
        .chunks_exact(2)
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .inspect(|v| assert!(*v < 32776)) // numbers 32776..65535 are invalid
        .collect()
}

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

    pub fn len(&self) -> u16 {
        self.mem.len() as u16
    }

    pub fn read(&self, a: u16) -> u16 {
        self.mem[a as usize]
    }

    pub fn write(&mut self, a: u16, val: u16) {
        self.mem[a as usize] = val;
    }

    pub fn ins_slice(&self, from: u16) -> &[u16] {
        let f = from as usize;
        let t = (self.mem.len() - 1).min(f + 4);
        &self.mem[f..t]
    }
}

// Holder for all 3 storage regions.
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
