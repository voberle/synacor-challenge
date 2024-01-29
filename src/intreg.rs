use crate::register::RegNb;

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
