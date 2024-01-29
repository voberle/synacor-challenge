use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::register::RegNb;
use crate::storage::Storage;
use crate::terminal::Terminal;

// add: 9 a b c
//   assign into <a> the sum of <b> and <c> (modulo 32768)
pub struct BinaryOp {
    name: &'static str,
    binary_fn: fn(u16, u16) -> u16,
    a: RegNb,
    b: IntReg,
    c: IntReg,
}

fn add(x: u16, y: u16) -> u16 {
    ((x as u32 + y as u32) % 32768) as u16
}

impl BinaryOp {
    fn new(name: &'static str, binary_fn: fn(u16, u16) -> u16, a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self {
            name,
            binary_fn,
            a,
            b,
            c,
        }
    }

    fn add(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("add", add, a, b, c)
    }

    pub fn inst_add<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let a = RegNb::from(*iter.next().unwrap());
        let b = IntReg::new(*iter.next().unwrap());
        let c = IntReg::new(*iter.next().unwrap());
        Box::new(Self::add(a, b, c))
    }

    fn sign(&self) -> &'static str {
        match self.name {
            "add" => "+",
            _ => panic!("Invalid name")
        }
    }
}

impl Instruction for BinaryOp {
    fn name(&self) -> &'static str {
        self.name
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(self.a, (self.binary_fn)(st.regs.get_ir(self.b), st.regs.get_ir(self.c)));
        *ir += 1;
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} = {} {} {}",
            self.name,
            self.a,
            self.b,
            self.sign(),
            self.c,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::register::RegNb;

    #[test]
    fn test_add() {
        let a = 16384;
        let b = 16384;
        assert_eq!(a + b, 32768);
        assert_eq!(add(a, b), 0);
    }
    #[test]
    fn test_exec_add() {
        let ins = BinaryOp::add(RegNb::new(3), IntReg::Register(RegNb::new(2)), IntReg::Value(37));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 40);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 77);
        assert_eq!(ir, 101);
    }
}
