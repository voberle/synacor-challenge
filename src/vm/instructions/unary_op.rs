use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::register::RegNb;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// not: 14 a b
//   stores 15-bit bitwise inverse of <b> in <a>
pub struct Not {
    a: RegNb,
    b: IntReg,
}

fn not(x: u16) -> u16 {
    !x & 0x07FFF
}

impl Not {
    const ARGS_COUNT: u16 = 2;

    fn new(a: RegNb, b: IntReg) -> Self {
        Self { a, b }
    }

    pub fn inst(mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        Box::new(Self::new(a, b))
    }
}

impl Instruction for Not {
    fn name(&self) -> &'static str {
        "not"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(self.a, not(st.regs.get_ir(self.b)));
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for Not {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not: {} = !{}", self.a, self.b)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::register::RegNb;

    #[test]
    fn test_not() {
        assert_eq!(not(0), 32767);
        assert_eq!(not(32767), 0);
        assert_eq!(not(4), 32763);
    }

    #[test]
    fn test_exec() {
        let ins = Not::new(RegNb::new(3), IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 4);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 32763);
        assert_eq!(ir, 103);
    }
}
