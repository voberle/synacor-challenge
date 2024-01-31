use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

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
    const ARGS_COUNT: u16 = 2;

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

    pub fn inst_jt(mem: &[u16]) -> Box<dyn Instruction> {
        let a = IntReg::new(mem[1]);
        let b = IntReg::new(mem[2]);
        Box::new(Self::jt(a, b))
    }

    pub fn inst_jf(mem: &[u16]) -> Box<dyn Instruction> {
        let a = IntReg::new(mem[1]);
        let b = IntReg::new(mem[2]);
        Box::new(Self::jf(a, b))
    }
}

impl Instruction for JumpIf {
    fn name(&self) -> &'static str {
        self.name
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        if (self.cond_fn)(st.regs.get_ir(self.a)) {
            *ir = st.regs.get_ir(self.b);
        } else {
            *ir += 1 + Self::ARGS_COUNT;
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
    use crate::vm::register::RegNb;

    #[test]
    fn test_exec_jt() {
        let ins = JumpIf::jt(IntReg::Register(RegNb::new(2)), IntReg::Value(37));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 0);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(ir, 103);

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
        assert_eq!(ir, 40);
    }
}
