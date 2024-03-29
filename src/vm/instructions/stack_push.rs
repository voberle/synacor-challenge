use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// push: 2 a
//   push <a> onto the stack
pub struct Push {
    addr: u16,
    a: IntReg,
}

impl Push {
    const ARGS_COUNT: u16 = 1;

    fn new(addr: u16, a: IntReg) -> Self {
        Self { addr, a }
    }

    pub fn inst(addr: u16, mem: &[u16]) -> Box<dyn Instruction> {
        let a = IntReg::new(mem[1]);
        Box::new(Self::new(addr, a))
    }
}

impl Instruction for Push {
    fn name(&self) -> &'static str {
        "push"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!("{}\t{}\t{}", self.addr, self.name(), self.a)
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.stack.push(st.regs.get_ir(self.a));
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for Push {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: Push {} onto stack", self.name(), self.a)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::register::RegNb;

    #[test]
    fn test_exec() {
        let ins1 = Push::new(1, IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 444);
        let mut ir = 100;
        ins1.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(*storage.stack.first().unwrap(), 444);
        assert_eq!(ir, 102);
    }
}
