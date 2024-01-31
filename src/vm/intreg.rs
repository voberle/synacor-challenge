use std::fmt;

use crate::vm::register::RegNb;

// The numbers in the binary format can mean two things: A literal value or a register number.
#[derive(Debug, Clone, Copy)]
pub enum IntReg {
    Value(u16),
    Register(RegNb),
}

impl IntReg {
    pub fn new(n: u16) -> Self {
        match n {
            0..=32767 => IntReg::Value(n),
            32768..=32775 => IntReg::Register(RegNb::from(n)),
            _ => panic!("Invalid number"),
        }
    }
}

impl fmt::Display for IntReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntReg::Value(v) => write!(f, "{}", v),
            IntReg::Register(r) => write!(f, "{}", r),
        }
    }
}
