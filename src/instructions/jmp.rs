use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::storage::Storage;
use crate::terminal::Terminal;

// jmp: 6 a
// jump to <a>
pub struct Jmp {
    a: IntReg,
}

impl Jmp {
    pub fn inst<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let v = *iter.next().unwrap();
        Box::new(Self { a: IntReg::new(v) })
    }
}

impl Instruction for Jmp {
    fn name(&self) -> &'static str {
        "jmp"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        *ir = st.regs.get_ir(self.a);
    }
}

impl fmt::Display for Jmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Jmp to {}", self.a)
    }
}
