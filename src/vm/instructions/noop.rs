use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// noop: 21
// no operation
pub struct Noop {
    addr: u16,
}

impl Noop {
    const ARGS_COUNT: u16 = 0;

    fn new(addr: u16) -> Self {
        Self { addr }
    }

    pub fn inst(addr: u16, _mem: &[u16]) -> Box<dyn Instruction> {
        Box::new(Self::new(addr))
    }
}

impl Instruction for Noop {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!("{}\t{}", self.addr, self.name())
    }

    fn exec(&self, ir: &mut u16, _st: &mut Storage, _term: &mut Terminal) {
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for Noop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Noop")
    }
}
