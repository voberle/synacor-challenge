use crate::instruction::IntReg;

fn register(v: u16) -> Option<u16> {
    if (32768..=32775).contains(&v) {
        return Some(v - 32768);
    }
    None
}

#[derive(Debug)]
pub struct Registers {
    regs: [u16; 8],
}

impl Registers {
    pub fn new() -> Self {
        Self { regs: [0; 8] }
    }

    pub fn get(&self, i: u16) -> u16 {
        self.regs[i as usize]
    }

    pub fn set(&mut self, i: u16, val: u16) {
        self.regs[i as usize] = val;
    }

    pub fn get_ir(&self, x: IntReg) -> u16 {
        match x {
            IntReg::Value(val) => val,
            IntReg::Register(r) => self.get(r),
        }
    }

    pub fn set_ir(&mut self, x: IntReg, val: u16) {
        match x {
            IntReg::Register(r) => {
                self.regs[r as usize] = val;
            }
            IntReg::Value(_) => {
                // Ignore, but weird..
            }
        }
    }

    pub fn binary_op(&mut self, a: IntReg, b: IntReg, c: IntReg, op_fn: fn(u16, u16) -> u16) {
        self.set_ir(a, op_fn(self.get_ir(b), self.get_ir(c)));
    }
}
