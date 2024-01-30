use crate::instructions::Instruction;
use crate::storage::Storage;
use crate::terminal::Terminal;
use std::fmt;

// noop: 21
// no operation
pub struct Noop {
    // Used to save the opcode when Noop is used to replaced an unimplemented instruction.
    opcode: u16,
}

impl Noop {
    const ARGS_COUNT: u16 = 0;

    fn new<const OPCODE: u16>() -> Self {
        Self { opcode: OPCODE }
    }

    pub fn inst<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        Box::new(Self::new::<OPCODE>())
    }
}

impl Instruction for Noop {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn exec(&self, ir: &mut u16, _st: &mut Storage, _term: &mut Terminal) {
        *ir += 1 + Self::ARGS_COUNT;
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
