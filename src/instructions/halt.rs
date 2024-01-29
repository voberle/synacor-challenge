use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::storage::Storage;
use crate::terminal::Terminal;

// halt: 0
// stop execution and terminate the program
pub struct Halt {}

impl Halt {
    fn new() -> Self {
        Self {}
    }

    pub fn inst<const OPCODE: u8>(_iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
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
