use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::register::RegNb;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// set: 1 a b
//   set register <a> to the value of <b>
pub struct Set {
    addr: u16,
    a: RegNb,
    b: IntReg,
}

impl Set {
    const ARGS_COUNT: u16 = 2;

    fn new(addr: u16, a: RegNb, b: IntReg) -> Self {
        Self { addr, a, b }
    }

    pub fn inst(addr: u16, mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        Box::new(Self::new(addr, a, b))
    }
}

impl Instruction for Set {
    fn name(&self) -> &'static str {
        "set"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!("{}\t{}\t{}\t{}", self.addr, self.name(), self.a, self.b)
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(self.a, st.regs.get_ir(self.b));
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Set: {} = {}", self.a, self.b)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::register::RegNb;

    #[test]
    fn test_exec_set() {
        let ins = Set::new(1, RegNb::new(3), IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 40);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 40);
        assert_eq!(ir, 103);
    }
}
