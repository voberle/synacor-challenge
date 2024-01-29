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
    fn new(a: IntReg) -> Self {
        Self { a }
    }

    pub fn inst<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let v = *iter.next().unwrap();
        Box::new(Self::new(IntReg::new(v)))
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
        write!(f, "{}: To {}", self.name(), self.a)
    }
}

#[test]
fn test_exec() {
    let ins = Jmp::new(IntReg::Value(37));
    let mut ir = 100;
    ins.exec(&mut ir, &mut Storage::new(), &mut Terminal::new(false));
    assert_eq!(ir, 37);
}
