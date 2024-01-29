use std::slice::Iter;

use crate::instructions::Instruction;
use crate::storage::Storage;
use crate::terminal::Terminal;

// noop: 21
// no operation
pub struct Noop {}

impl Noop {
    pub fn new(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        Box::new(Self {})
    }
}

impl Instruction for Noop {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal) {
        *ir += 1;
    }
}
