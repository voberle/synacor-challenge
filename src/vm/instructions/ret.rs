use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// ret: 18
//   remove the top element from the stack and jump to it; empty stack = halt
pub struct Ret {
    addr: u16,
}

impl Ret {
    const ARGS_COUNT: u16 = 0;

    fn new(addr: u16) -> Self {
        Self { addr }
    }

    pub fn inst(addr: u16, _mem: &[u16]) -> Box<dyn Instruction> {
        Box::new(Self::new(addr))
    }
}

impl Instruction for Ret {
    fn name(&self) -> &'static str {
        "ret"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!("{}\t{}", self.addr, self.name())
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        let address = st.stack.pop().expect("Stack is empty");
        *ir = address;
    }
}

impl fmt::Display for Ret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: Take top from stack and jump to it", self.name())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec() {
        let ins = Ret::new(1);
        let mut storage = Storage::new();
        storage.stack.push(300);
        storage.stack.push(478);
        let mut ir = 100;
        ins.exec(&mut ir, &mut storage, &mut Terminal::new(false));
        assert_eq!(ir, 478);
    }
}
