use std::fmt;
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
    fn new(a: IntReg) -> Self {
        Self { a }
    }

    pub fn inst<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let v = *iter.next().unwrap();
        Box::new(Self::new(IntReg::new(v)))
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
