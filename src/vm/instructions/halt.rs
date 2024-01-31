use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// halt: 0
// stop execution and terminate the program
pub struct Halt {
    addr: u16,
}

impl Halt {
    const ARGS_COUNT: u16 = 0;

    fn new(addr: u16) -> Self {
        Self { addr }
    }

    pub fn inst(addr: u16, _mem: &[u16]) -> Box<dyn Instruction> {
        Box::new(Self::new(addr))
    }
}

impl Instruction for Halt {
    fn name(&self) -> &'static str {
        "halt"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
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
