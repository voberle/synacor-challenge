use std::fmt;

use crate::instructions::Instruction;
use crate::storage::Storage;
use crate::terminal::Terminal;

// halt: 0
// stop execution and terminate the program
pub struct Halt {}

impl Halt {
    const _ARGS_COUNT: u16 = 0;

    fn new() -> Self {
        Self {}
    }

    pub fn inst<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
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
