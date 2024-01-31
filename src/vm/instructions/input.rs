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
    addr: u16,
    a: RegNb,
}

impl In {
    const ARGS_COUNT: u16 = 1;

    fn new(addr: u16, a: RegNb) -> Self {
        Self { addr, a }
    }

    pub fn inst(addr: u16, mem: &[u16]) -> Box<dyn Instruction> {
        let a = RegNb::from(mem[1]);
        Box::new(Self::new(addr, a))
    }
}

impl Instruction for In {
    fn name(&self) -> &'static str {
        "in"
    }

    fn offset(&self) -> u16 {
        1 + Self::ARGS_COUNT
    }

    fn decompile(&self) -> String {
        format!("{}\t{}\t{}", self.addr, self.name(), self.a)
    }

    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal) {
        if let Some(c) = term.read() {
            st.regs.set(self.a, c as u16);
            *ir += 1 + Self::ARGS_COUNT;
        } else {
            assert!(term.is_interactive_mode());
            // By not modifying ir in case read returned None, we ensure that next exec attempt will try on this instruction again.
        }
    }
}

impl fmt::Display for In {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "In: {}", self.a)
    }
}
