use std::fmt;
use std::slice::Iter;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::storage::Storage;
use crate::terminal::Terminal;

// call: 17 a
//   write the address of the next instruction to the stack and jump to <a>
pub struct Call {
    a: IntReg,
}

impl Call {
    fn new(a: IntReg) -> Self {
        Self { a }
    }

    pub fn inst<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
        let v = *iter.next().unwrap();
        Box::new(Self::new(IntReg::new(v)))
    }
}

impl Instruction for Call {
    fn name(&self) -> &'static str {
        "call"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.stack.push(*ir + 1);
        *ir = st.regs.get_ir(self.a);
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: Write next ir to stack and jump to {}",
            self.name(),
            self.a
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec() {
        let ins = Call::new(IntReg::Value(37));
        let mut storage = Storage::new();
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut Terminal::new(false));
        assert_eq!(*storage.stack.first().unwrap(), 101);
        assert_eq!(ir, 37);
    }
}
