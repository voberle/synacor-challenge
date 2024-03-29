use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// jmp: 6 a
// jump to <a>
pub struct Jmp {
    addr: u16,
    a: IntReg,
}

impl Jmp {
    const ARGS_COUNT: u16 = 1;

    fn new(addr: u16, a: IntReg) -> Self {
        Self { addr, a }
    }

    pub fn inst(addr: u16, mem: &[u16]) -> Box<dyn Instruction> {
        let a = IntReg::new(mem[1]);
        Box::new(Self::new(addr, a))
    }
}

impl Instruction for Jmp {
    fn name(&self) -> &'static str {
        "jmp"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!("{}\t{}\t{}", self.addr, self.name(), self.a)
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
    let ins = Jmp::new(1, IntReg::Value(37));
    let mut ir = 100;
    ins.exec(&mut ir, &mut Storage::new(), &mut Terminal::new(false));
    assert_eq!(ir, 37);
}
