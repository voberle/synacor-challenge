use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::intreg::IntReg;
use crate::vm::register::RegNb;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// add: 9 a b c
//   assign into <a> the sum of <b> and <c> (modulo 32768)
// mult: 10 a b c
//  store into <a> the product of <b> and <c> (modulo 32768)
// mod: 11 a b c
//   store into <a> the remainder of <b> divided by <c>
// and: 12 a b c
//   stores into <a> the bitwise and of <b> and <c>
// or: 13 a b c
//   stores into <a> the bitwise or of <b> and <c>
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

fn mult(x: u16, y: u16) -> u16 {
    ((x as u32 * y as u32) % 32768) as u16
}

fn modulo(x: u16, y: u16) -> u16 {
    x % y
}

fn and(x: u16, y: u16) -> u16 {
    x & y
}

fn or(x: u16, y: u16) -> u16 {
    x | y
}

impl BinaryOp {
    const ARGS_COUNT: u16 = 3;

    fn new(
        name: &'static str,
        binary_fn: fn(u16, u16) -> u16,
        a: RegNb,
        b: IntReg,
        c: IntReg,
    ) -> Self {
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

    fn mult(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("mult", mult, a, b, c)
    }

    fn modulo(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("mod", modulo, a, b, c)
    }

    fn and(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("and", and, a, b, c)
    }

    fn or(a: RegNb, b: IntReg, c: IntReg) -> Self {
        Self::new("or", or, a, b, c)
    }

    pub fn inst_add(mem: &[u16]) -> Box<dyn Instruction> {
        // For "add", spec says "assign into <a>", while for the other operations
        // it says "store into <a>".
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        let c = IntReg::new(mem[3]);
        Box::new(Self::add(a, b, c))
    }

    pub fn inst_mult(mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        let c = IntReg::new(mem[3]);
        Box::new(Self::mult(a, b, c))
    }

    pub fn inst_mod(mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        let c = IntReg::new(mem[3]);
        Box::new(Self::modulo(a, b, c))
    }

    pub fn inst_and(mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        let c = IntReg::new(mem[3]);
        Box::new(Self::and(a, b, c))
    }

    pub fn inst_or(mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        let b = IntReg::new(mem[2]);
        let c = IntReg::new(mem[3]);
        Box::new(Self::or(a, b, c))
    }

    fn sign(&self) -> &'static str {
        match self.name {
            "add" => "+",
            "mult" => "*",
            "mod" => "%",
            "and" => "&",
            "or" => "|",
            _ => panic!("Invalid name"),
        }
    }
}

impl Instruction for BinaryOp {
    fn name(&self) -> &'static str {
        self.name
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(
            self.a,
            (self.binary_fn)(st.regs.get_ir(self.b), st.regs.get_ir(self.c)),
        );
        *ir += 1 + Self::ARGS_COUNT;
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
    use crate::vm::register::RegNb;

    #[test]
    fn test_add() {
        let a = 16384;
        let b = 16384;
        assert_eq!(a + b, 32768);
        assert_eq!(add(a, b), 0);
        // Adding 32767 is same as -1
        assert_eq!(add(10, 32767), 9);
        assert_eq!(add(45, 32767), 44);
    }

    #[test]
    fn test_exec_add() {
        let ins = BinaryOp::add(
            RegNb::new(3),
            IntReg::Register(RegNb::new(2)),
            IntReg::Value(37),
        );
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 40);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 77);
        assert_eq!(ir, 104);
    }

    #[test]
    fn test_exec_mult() {
        let ins = BinaryOp::mult(
            RegNb::new(3),
            IntReg::Value(10),
            IntReg::Register(RegNb::new(4)),
        );
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(4), 420);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 4200);
        assert_eq!(ir, 104);
    }

    #[test]
    fn test_exec_mod() {
        let ins = BinaryOp::modulo(
            RegNb::new(3),
            IntReg::Value(37),
            IntReg::Register(RegNb::new(4)),
        );
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(4), 3);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 1);
        assert_eq!(ir, 104);
    }

    #[test]
    fn test_exec_and() {
        let ins = BinaryOp::and(RegNb::new(3), IntReg::Value(3), IntReg::Value(5));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 1);
        assert_eq!(ir, 104);
    }
}
