use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::storage::Storage;
use crate::terminal::Terminal;

// jt: 7 a b
//   if <a> is nonzero, jump to <b>
// jf: 8 a b
//   if <a> is zero, jump to <b>
pub struct JumpIf {
    name: &'static str,
    cond_fn: fn(u16) -> bool,
    a: IntReg,
    b: IntReg,
}

impl JumpIf {
    fn new(name: &'static str, cond_fn: fn(u16) -> bool, a: IntReg, b: IntReg) -> Self {
        Self {
            name,
            cond_fn,
            a,
            b,
        }
    }

    fn jt(a: IntReg, b: IntReg) -> Self {
        Self::new("jt", |v| v != 0, a, b)
    }

    fn jf(a: IntReg, b: IntReg) -> Self {
        Self::new("jf", |v| v == 0, a, b)
    }

    pub fn inst_jt<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let a = IntReg::new(*iter.next().unwrap());
        let b = IntReg::new(*iter.next().unwrap());
        Box::new(Self::jt(a, b))
    }

    pub fn inst_jf<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let a = IntReg::new(*iter.next().unwrap());
        let b = IntReg::new(*iter.next().unwrap());
        Box::new(Self::jf(a, b))
    }
}

impl Instruction for JumpIf {
    fn name(&self) -> &'static str {
        self.name
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        if (self.cond_fn)(st.regs.get_ir(self.a)) {
            *ir = st.regs.get_ir(self.b);
        } else {
            *ir += 1;
        }
    }
}

impl fmt::Display for JumpIf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: Jump to {} if {} {}",
            self.name,
            self.b,
            self.a,
            if self.name == "jt" { "!= 0" } else { "== 0" }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::register::RegNb;

    #[test]
    fn test_exec_jt() {
        let ins = JumpIf::jt(IntReg::Register(RegNb::new(2)), IntReg::Value(37));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 0);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(ir, 101);

        storage.regs.set(RegNb::new(2), 45);
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(ir, 37);
    }

    #[test]
    fn test_exec_jf() {
        let ins = JumpIf::jf(IntReg::Register(RegNb::new(2)), IntReg::Value(37));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 0);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(ir, 37);

        storage.regs.set(RegNb::new(2), 45);
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(ir, 38);
    }
}
