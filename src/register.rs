use std::{fmt, ops::Deref};

use crate::instruction::IntReg;

// Number of a register. Enforces that the registers number is in correct range,
// and provides helper functions such as more readable display.
#[derive(Debug, Clone, Copy)]
pub struct RegNb {
    value: usize,
}

impl RegNb {
    fn new(value: usize) -> Self {
        assert!((0..=7).contains(&value));
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

    fn get(&self, i: u16) -> u16 {
        self.regs[i as usize]
    }

    fn set(&mut self, i: u16, val: u16) {
        self.regs[i as usize] = val;
    }

    pub fn get_ir(&self, x: IntReg) -> u16 {
        match x {
            IntReg::Value(val) => val,
            IntReg::Register(r) => self.regs[*r],
        }
    }

    pub fn set_ir(&mut self, x: IntReg, val: u16) {
        match x {
            IntReg::Register(r) => {
                self.regs[*r] = val;
            }
            IntReg::Value(_) => {
                // Ignore, but weird..
            }
        }
    }

    pub fn cmp_op(&mut self, a: IntReg, b: IntReg, c: IntReg, cmp_fn: fn(u16, u16) -> bool) {
        self.set_ir(
            a,
            if cmp_fn(self.get_ir(b), self.get_ir(c)) {
                1
            } else {
                0
            },
        );
    }

    pub fn binary_op(&mut self, a: IntReg, b: IntReg, c: IntReg, binary_fn: fn(u16, u16) -> u16) {
        self.set_ir(a, binary_fn(self.get_ir(b), self.get_ir(c)));
    }

    pub fn unary_op(&mut self, a: IntReg, b: IntReg, unary_fn: fn(u16) -> u16) {
        self.set_ir(a, unary_fn(self.get_ir(b)));
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.regs)
    }
}
