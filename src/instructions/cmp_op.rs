use std::fmt;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::register::RegNb;
use crate::storage::Storage;
use crate::terminal::Terminal;

// eq: 4 a b c
//   set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
// gt: 5 a b c
//   set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
pub struct CmpOp {
    name: &'static str,
    cmp_fn: fn(u16, u16) -> bool,
    a: RegNb,
    b: IntReg,
    c: IntReg,
}

fn eq(x: u16, y: u16) -> bool {
    x == y
}

fn gt(x: u16, y: u16) -> bool {
    x > y
}

impl CmpOp {
    const ARGS_COUNT: u16 = 3;

    fn new(
        name: &'static str,
        cmp_fn: fn(u16, u16) -> bool,
        a: RegNb,
        b: IntReg,
        c: IntReg,
    ) -> Self {
        Self {
            name,
            cmp_fn,
            a,
            b,
            c,
        }
    }

    fn eq(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("eq", eq, a, b, c)
    }

    fn gt(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("gt", gt, a, b, c)
    }

    pub fn inst_eq<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        let a = RegNb::from(storage.mem.read(address + 1));
        let b = IntReg::new(storage.mem.read(address + 2));
        let c = IntReg::new(storage.mem.read(address + 3));
        Box::new(Self::eq(a, b, c))
    }

    pub fn inst_gt<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        let a = RegNb::from(storage.mem.read(address + 1));
        let b = IntReg::new(storage.mem.read(address + 2));
        let c = IntReg::new(storage.mem.read(address + 3));
        Box::new(Self::gt(a, b, c))
    }

    fn sign(&self) -> &'static str {
        match self.name {
            "eq" => "==",
            "gt" => ">",
            _ => panic!("Invalid name"),
        }
    }
}

impl Instruction for CmpOp {
    fn name(&self) -> &'static str {
        self.name
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(
            self.a,
            if (self.cmp_fn)(st.regs.get_ir(self.b), st.regs.get_ir(self.c)) {
                1
            } else {
                0
            },
        );
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} = 1 if {} {} {}",
            self.name(),
            self.a,
            self.b,
            self.sign(),
            self.c
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::register::RegNb;

    #[test]
    fn test_exec_eq() {
        let ins = CmpOp::eq(RegNb::new(0), IntReg::Value(2), IntReg::Value(2));
        let mut ir = 100;
        let mut storage = Storage::new();
        ins.exec(&mut ir, &mut storage, &mut Terminal::new(false));
        assert_eq!(storage.regs.get(RegNb::new(0)), 1);
    }

    #[test]
    fn test_exec_gt() {
        let ins = CmpOp::gt(RegNb::new(0), IntReg::Value(20), IntReg::Value(2));
        let mut ir = 100;
        let mut storage = Storage::new();
        ins.exec(&mut ir, &mut storage, &mut Terminal::new(false));
        assert_eq!(storage.regs.get(RegNb::new(0)), 1);
    }
}
