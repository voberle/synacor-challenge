use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// jmp: 6 a
// jump to <a>
pub struct Jmp {
    a: IntReg,
}

impl Jmp {
    const _ARGS_COUNT: u16 = 1;

    fn new(a: IntReg) -> Self {
        Self { a }
    }

    pub fn inst(mem: &[u16]) -> Box<dyn Instruction> {
        let a = IntReg::new(mem[1]);
        Box::new(Self::new(a))
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
