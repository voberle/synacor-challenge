use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::register::RegNb;
use crate::storage::Storage;
use crate::terminal::Terminal;

// eq: 4 a b c
// set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
pub struct Eq {
    a: RegNb,
    b: IntReg,
    c: IntReg,
}

impl Eq {
    fn new(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self { a, b, c }
    }

    pub fn inst<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let a = RegNb::from(*iter.next().unwrap());
        let b = IntReg::new(*iter.next().unwrap());
        let c = IntReg::new(*iter.next().unwrap());
        Box::new(Self::new(a, b, c))
    }
}

impl Instruction for Eq {
    fn name(&self) -> &'static str {
        "eq"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.cmp_op(self.a, self.b, self.c, |x, y| x == y);
        *ir += 1;
    }
}

impl fmt::Display for Eq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} = 1 if {} == {}", self.name(), self.a, self.b, self.c)
    }
}

#[test]
fn test_exec() {
    let ins = Eq::new(RegNb::new(0), IntReg::Value(2), IntReg::Value(2));
    let mut ir = 100;
    let mut storage = Storage::new();
    ins.exec(&mut ir, &mut storage, &mut Terminal::new(false));
    assert_eq!(storage.regs.get(RegNb::new(0)), 1);
}
