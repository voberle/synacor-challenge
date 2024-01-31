use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// out: 19 a
// write the character represented by ascii code <a> to the terminal
pub struct Out {
    a: IntReg,
}

impl Out {
    const ARGS_COUNT: u16 = 1;

    fn new(a: IntReg) -> Self {
        Self { a }
    }

    pub fn inst<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        let a = IntReg::new(storage.mem.read(address + 1));
        Box::new(Self::new(a))
    }
}

impl Instruction for Out {
    fn name(&self) -> &'static str {
        "out"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal) {
        term.write(st.regs.get_ir(self.a) as u8 as char);
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for Out {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.a {
            IntReg::Value(v) => {
                let c = v as u8 as char;
                write!(
                    f,
                    "Out: {} ({})",
                    self.a,
                    if c == '\n' {
                        "\\n".to_string()
                    } else {
                        c.to_string()
                    }
                )
            }
            IntReg::Register(r) => write!(f, "Out: {} ({})", self.a, r),
        }
    }
}