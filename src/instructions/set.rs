use std::fmt;
use std::slice::Iter;

use crate::instructions::{noop, Instruction};
use crate::intreg::IntReg;
use crate::register::RegNb;
use crate::storage::Storage;
use crate::terminal::Terminal;

// set: 1 a b
//   set register <a> to the value of <b>
pub struct Set {
    a: RegNb,
    b: IntReg,
}

impl Set {
    fn new(a: RegNb, b: IntReg) -> Self {
        Self { a, b }
    }

    pub fn inst<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let a_val = *iter.next().unwrap();
        if a_val < 32768 {
            eprintln!("Warning: <a> for or is not a register: {}", a_val);
            // Skipping for now. IS IT THE RIGHT THING TO DO?
            iter.next();
            iter.next();
            return noop::Noop::inst::<OPCODE>(iter);
        }
        let a = RegNb::from(a_val);
        let b = IntReg::new(*iter.next().unwrap());
        Box::new(Self::new(a, b))
    }
}

impl Instruction for Set {
    fn name(&self) -> &'static str {
        "set"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(self.a, st.regs.get_ir(self.b));
        *ir += 1;
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
    use crate::register::RegNb;

    #[test]
    fn test_exec_set() {
        let ins = Set::new(RegNb::new(3), IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 40);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 40);
        assert_eq!(ir, 101);
    }
}
