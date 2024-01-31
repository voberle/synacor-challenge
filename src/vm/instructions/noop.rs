use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// noop: 21
// no operation
pub struct Noop {}

impl Noop {
    const ARGS_COUNT: u16 = 0;

    fn new() -> Self {
        Self {}
    }

    pub fn inst(_mem: &[u16]) -> Box<dyn Instruction> {
        Box::new(Self::new())
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
        write!(f, "Noop")
    }
}
