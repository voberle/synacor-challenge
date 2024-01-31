use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::register::RegNb;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// rmem: 15 a b
//   read memory at address <b> and write it to <a>
pub struct RMem {
    a: RegNb,
    b: IntReg,
}

impl RMem {
    const ARGS_COUNT: u16 = 2;

    fn new(a: RegNb, b: IntReg) -> Self {
        Self { a, b }
    }

    pub fn inst<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        let a = RegNb::from(storage.mem.read(address + 1));
        let b = IntReg::new(storage.mem.read(address + 2));
        Box::new(Self::new(a, b))
    }
}

impl Instruction for RMem {
    fn name(&self) -> &'static str {
        "rmem"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(self.a, st.mem.read(st.regs.get_ir(self.b)));
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for RMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: Read at {} and write to {}",
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
    fn test_exec_rmem() {
        let ins = RMem::new(RegNb::new(2), IntReg::Value(1000));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.mem.write(1000, 567);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(2)), 567);
        assert_eq!(ir, 103);
    }
}
