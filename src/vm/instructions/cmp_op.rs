use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::register::RegNb;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// eq: 4 a b c
//   set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
// gt: 5 a b c
//   set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
pub struct CmpOp {
    name: &'static str,
    cmp_fn: fn(u16, u16) -> bool,
    addr: u16,
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
        addr: u16,
        a: RegNb,
        b: IntReg,
        c: IntReg,
    ) -> Self {
        Self {
            name,
            cmp_fn,
            addr,
            a,
            b,
            c,
        }
    }

    fn eq(addr: u16, a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("eq", eq, addr, a, b, c)
    }

    fn gt(addr: u16, a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("gt", gt, addr, a, b, c)
    }

    pub fn inst_eq(addr: u16, mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        let c = IntReg::new(mem[3]);
        Box::new(Self::eq(addr, a, b, c))
    }

    pub fn inst_gt(addr: u16, mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        let c = IntReg::new(mem[3]);
        Box::new(Self::gt(addr, a, b, c))
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

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.addr,
            self.name(),
            self.a,
            self.b,
            self.c
        )
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
    use crate::vm::register::RegNb;

    #[test]
    fn test_exec_eq() {
        let ins = CmpOp::eq(1, RegNb::new(0), IntReg::Value(2), IntReg::Value(2));
        let mut ir = 100;
        let mut storage = Storage::new();
        ins.exec(&mut ir, &mut storage, &mut Terminal::new(false));
        assert_eq!(storage.regs.get(RegNb::new(0)), 1);
    }

    #[test]
    fn test_exec_gt() {
        let ins = CmpOp::gt(1, RegNb::new(0), IntReg::Value(20), IntReg::Value(2));
        let mut ir = 100;
        let mut storage = Storage::new();
        ins.exec(&mut ir, &mut storage, &mut Terminal::new(false));
        assert_eq!(storage.regs.get(RegNb::new(0)), 1);
    }
}
