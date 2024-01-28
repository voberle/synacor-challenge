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

    pub fn cmp_op(&mut self, a: IntReg, b: IntReg, c: IntReg, cmp_fn: fn(u16, u16) -> bool) {
        self.set_ir(
            a,
            if cmp_fn(self.get_ir(b), self.get_ir(c)) {
                1
            } else {
                0
            },
        );
    }

    pub fn binary_op(&mut self, a: IntReg, b: IntReg, c: IntReg, binary_fn: fn(u16, u16) -> u16) {
        self.set_ir(a, binary_fn(self.get_ir(b), self.get_ir(c)));
    }

    pub fn unary_op(&mut self, a: IntReg, b: IntReg, unary_fn: fn(u16) -> u16) {
        self.set_ir(a, unary_fn(self.get_ir(b)));
    }
}

pub struct Storage {
    pub regs: Registers,
    pub stack: Vec<u16>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            stack: Vec::new(),
        }
    }
}
