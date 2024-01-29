use std::slice::Iter;

use crate::instructions::Instruction;
use crate::storage::Storage;
use crate::terminal::Terminal;

// halt: 0
// stop execution and terminate the program
pub struct Halt {}

impl Halt {
    pub fn new(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        Box::new(Self {})
    }
}

impl Instruction for Halt {
    fn name(&self) -> &'static str {
        "halt"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal) {
        std::process::exit(0);
    }
}
