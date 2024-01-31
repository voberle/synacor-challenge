use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// halt: 0
// stop execution and terminate the program
pub struct Halt {}

impl Halt {
    const _ARGS_COUNT: u16 = 0;

    fn new() -> Self {
        Self {}
    }

    pub fn inst(_mem: &[u16]) -> Box<dyn Instruction> {
        Box::new(Self::new())
    }
}

impl Instruction for Halt {
    fn name(&self) -> &'static str {
        "halt"
    }

    fn exec(&self, _ir: &mut u16, _st: &mut Storage, _term: &mut Terminal) {
        println!("Halting");
        std::process::exit(0);
    }
}

impl fmt::Display for Halt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Halt")
    }
}
