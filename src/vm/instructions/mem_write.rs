use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// wmem: 16 a b
//   write the value from <b> into memory at address <a>
pub struct WMem {
    a: IntReg,
    b: IntReg,
}

impl WMem {
    const ARGS_COUNT: u16 = 2;

    fn new(a: IntReg, b: IntReg) -> Self {
        Self { a, b }
    }

    pub fn inst(mem: &[u16]) -> Box<dyn Instruction> {
        let a = IntReg::new(mem[1]);
        let b = IntReg::new(mem[2]);
        Box::new(Self::new(a, b))
    }
}

impl Instruction for WMem {
    fn name(&self) -> &'static str {
        "wmem"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.mem.write(st.regs.get_ir(self.a), st.regs.get_ir(self.b));
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for WMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: Write val of {} into {}",
            self.name(),
            self.b,
            self.a
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::register::RegNb;

    #[test]
    fn test_exec_wmem() {
        let ins = WMem::new(IntReg::Value(1000), IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 8660);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.mem.read(1000), 8660);
        assert_eq!(ir, 103);
    }
}
