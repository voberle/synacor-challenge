use std::slice::Iter;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::storage::Storage;
use crate::terminal::Terminal;

// out: 19 a
// write the character represented by ascii code <a> to the terminal
pub struct Out {
    a: IntReg,
}

impl Out {
    pub fn new(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let v = *iter.next().unwrap();
        Box::new(Self { a: IntReg::new(v) })
    }
}

impl Instruction for Out {
    fn name(&self) -> &'static str {
        "out"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal) {
        term.write(st.regs.get_ir(self.a) as u8 as char);
        *ir += 1;
    }
}