use std::{fmt, ops::Deref};

use crate::vm::intreg::IntReg;

// Number of a register. Enforces that the registers number is in correct range,
// and provides helper functions such as more readable display.
#[derive(Debug, Clone, Copy)]
pub struct RegNb {
    value: usize,
}

impl RegNb {
    pub fn is_valid(value: usize) -> bool {
        (0..=7).contains(&value)
    }

    pub fn new(value: usize) -> Self {
        assert!(Self::is_valid(value));
        Self { value }
    }
}

impl From<u16> for RegNb {
    fn from(item: u16) -> Self {
        assert!((32768..=32775).contains(&item));
        RegNb::new((item - 32768) as usize)
    }
}

impl Deref for RegNb {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl fmt::Display for RegNb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}", self.value)
    }
}

#[derive(Debug)]
pub struct Registers {
    regs: [u16; 8],
}

impl Registers {
    pub fn new() -> Self {
        Self { regs: [0; 8] }
    }

    pub fn set(&mut self, i: RegNb, val: u16) {
        self.regs[*i] = val;
    }

    pub fn get_ir(&self, x: IntReg) -> u16 {
        match x {
            IntReg::Value(val) => val,
            IntReg::Register(r) => self.regs[*r],
        }
    }

    #[cfg(test)]
    pub fn get(&self, i: RegNb) -> u16 {
        self.regs[*i]
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.regs)
    }
}
