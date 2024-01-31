use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// wmem: 16 a b
//   write the value from <b> into memory at address <a>
pub struct WMem {
    addr: u16,
    a: IntReg,
    b: IntReg,
}

impl WMem {
    const ARGS_COUNT: u16 = 2;

    fn new(addr: u16, a: IntReg, b: IntReg) -> Self {
        Self { addr, a, b }
    }

    pub fn inst(addr: u16, mem: &[u16]) -> Box<dyn Instruction> {
        let a = IntReg::new(mem[1]);
        let b = IntReg::new(mem[2]);
        Box::new(Self::new(addr, a, b))
    }
}

impl Instruction for WMem {
    fn name(&self) -> &'static str {
        "wmem"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!("{}\t{}\t{}\t{}", self.addr, self.name(), self.a, self.b)
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
        let ins = WMem::new(1, IntReg::Value(1000), IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 8660);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.mem.read(1000), 8660);
        assert_eq!(ir, 103);
    }
}
