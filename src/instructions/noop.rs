use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::storage::Storage;
use crate::terminal::Terminal;

// noop: 21
// no operation
pub struct Noop {
    // Used to save the opcode when Noop is used to replaced an unimplemented instruction.
    opcode: u8,
}

impl Noop {
    fn new<const OPCODE: u8>() -> Self {
        Self { opcode: OPCODE }
    }

    pub fn inst<const OPCODE: u8>(_iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        Box::new(Self::new::<OPCODE>())
    }
}

impl Instruction for Noop {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn exec(&self, ir: &mut u16, _st: &mut Storage, _term: &mut Terminal) {
        *ir += 1;
    }
}

impl fmt::Display for Noop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.opcode == 21 {
            write!(f, "Noop")
        } else {
            write!(f, "NOT IMPL: {}", self.opcode)
        }
    }
}
