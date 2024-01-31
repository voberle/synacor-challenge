use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::register::RegNb;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// pop: 3 a
//   remove the top element from the stack and write it into <a>; empty stack = error
pub struct Pop {
    a: RegNb,
}

impl Pop {
    const ARGS_COUNT: u16 = 1;

    fn new(a: RegNb) -> Self {
        Self { a }
    }

    pub fn inst(mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        Box::new(Self::new(a))
    }
}

impl Instruction for Pop {
    fn name(&self) -> &'static str {
        "pop"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        st.regs.set(self.a, st.stack.pop().expect("Stack is empty"));
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for Pop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: Pop from stack to {}", self.name(), self.a,)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vm::register::RegNb;

    #[test]
    fn test_exec() {
        let ins2 = Pop::new(RegNb::new(3));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.stack.push(444);
        let mut ir = 100;
        ins2.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 444);
        assert_eq!(ir, 102);
    }
}
