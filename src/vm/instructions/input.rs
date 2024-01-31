use std::fmt;

use crate::vm::instructions::Instruction;
use crate::vm::register::RegNb;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

// in: 20 a
//   read a character from the terminal and write its ascii code to <a>;
// it can be assumed that once input starts, it will continue until
// a newline is encountered; this means that you can safely read whole
// lines from the keyboard instead of having to figure out how to read
// individual characters
pub struct In {
    a: RegNb,
}

impl In {
    const ARGS_COUNT: u16 = 1;

    fn new(a: RegNb) -> Self {
        Self { a }
    }

    pub fn inst<const OPCODE: u16>(storage: &Storage, address: u16) -> Box<dyn Instruction> {
        assert_eq!(storage.mem.read(address), OPCODE);
        let a = RegNb::from(storage.mem.read(address + 1));
        Box::new(Self::new(a))
    }
}

impl Instruction for In {
    fn name(&self) -> &'static str {
        "in"
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal) {
        let c = term.read() as u16;
        st.regs.set(self.a, c);
        *ir += 1 + Self::ARGS_COUNT;
    }
}

impl fmt::Display for In {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "In: {}", self.a)
    }
}
