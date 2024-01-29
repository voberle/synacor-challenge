use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::storage::Storage;
use crate::terminal::Terminal;

// rmem: 15 a b
//   read memory at address <b> and write it to <a>
// wmem: 16 a b
//   write the value from <b> into memory at address <a>
pub struct MemAccess {
    name: &'static str,
    write: bool, // false it's rmem, true it's wmem
    a: IntReg,
    b: IntReg,
}

impl MemAccess {
    fn new(name: &'static str, write: bool, a: IntReg, b: IntReg) -> Self {
        Self { name, write, a, b }
    }

    fn rmem(a: IntReg, b: IntReg) -> Self {
        Self::new("rmem", false, a, b)
    }

    fn wmem(a: IntReg, b: IntReg) -> Self {
        Self::new("wmem", true, a, b)
    }

    pub fn inst_rmem<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let a = IntReg::new(*iter.next().unwrap());
        let b = IntReg::new(*iter.next().unwrap());
        Box::new(Self::rmem(a, b))
    }

    pub fn inst_wmem<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let a = IntReg::new(*iter.next().unwrap());
        let b = IntReg::new(*iter.next().unwrap());
        Box::new(Self::wmem(a, b))
    }
}

impl Instruction for MemAccess {
    fn name(&self) -> &'static str {
        self.name
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        if self.write {
            st.mem.write(st.regs.get_ir(self.a), st.regs.get_ir(self.b));
        } else {
            st.regs.set_ir(self.a, st.mem.read(st.regs.get_ir(self.b)));
        }
        *ir += 1;
    }
}

impl fmt::Display for MemAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.write {
            write!(f, "{}: Write val of {} into {}", self.name, self.b, self.a)
        } else {
            write!(
                f,
                "{}: Read at {} and write to {}",
                self.name, self.b, self.a
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::register::RegNb;

    #[test]
    fn test_exec_rmem() {
        let ins = MemAccess::rmem(IntReg::Register(RegNb::new(2)), IntReg::Value(1000));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.mem.write(1000, 567);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(2)), 567);
        assert_eq!(ir, 101);
    }

    #[test]
    fn test_exec_wmem() {
        let ins = MemAccess::wmem(IntReg::Value(1000), IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 8660);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.mem.read(1000), 8660);
        assert_eq!(ir, 101);
    }
}
