use std::fmt;

use crate::instructions::Instruction;
use crate::intreg::IntReg;
use crate::storage::Storage;
use crate::terminal::Terminal;

// push: 2 a
//   push <a> onto the stack
// pop: 3 a
//   remove the top element from the stack and write it into <a>; empty stack = error
pub struct Stack {
    name: &'static str,
    pop: bool, // false means push, true means pop
    a: IntReg,
}

impl Stack {
    const ARGS_COUNT: u16 = 1;

    fn new(name: &'static str, pop: bool, a: IntReg) -> Self {
        Self { name, pop, a }
    }

    fn push(a: IntReg) -> Self {
        Self::new("push", false, a)
    }

    fn pop(a: IntReg) -> Self {
        Self::new("pop", true, a)
    }

    pub fn inst_push<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        let a = IntReg::new(storage.mem.read(address + 1));
        Box::new(Self::push(a))
    }

    pub fn inst_pop<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        let a = IntReg::new(storage.mem.read(address + 1));
        Box::new(Self::pop(a))
    }
}

impl Instruction for Stack {
    fn name(&self) -> &'static str {
        self.name
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, _term: &mut Terminal) {
        if self.pop {
            st.regs
                .set_ir(self.a, st.stack.pop().expect("Stack is empty"));
        } else {
            st.stack.push(st.regs.get_ir(self.a));
        }
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.pop {
            write!(f, "{}: Pop from stack to {}", self.name, self.a,)
        } else {
            write!(f, "{}: Push {} onto stack", self.name, self.a,)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::register::RegNb;

    #[test]
    fn test_exec_push_pop() {
        let ins1 = Stack::push(IntReg::Register(RegNb::new(2)));
        let mut terminal = Terminal::new(false);
        let mut storage = Storage::new();
        storage.regs.set(RegNb::new(2), 444);
        let mut ir = 100;
        ins1.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(*storage.stack.first().unwrap(), 444);
        assert_eq!(ir, 102);

        let ins2 = Stack::pop(IntReg::Register(RegNb::new(3)));
        ins2.exec(&mut ir, &mut storage, &mut terminal);
        assert_eq!(storage.regs.get(RegNb::new(3)), 444);
        assert_eq!(ir, 104);
    }
}
