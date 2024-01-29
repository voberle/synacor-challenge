mod binary;
mod codes;
mod instruction;
mod register;
mod storage;

use binary::load_bin;
use instruction::{Instruction, IntReg};
use register::RegNb;
use storage::Storage;

fn execute(ins: &Instruction, ir: &mut u16, st: &mut Storage) {
    match *ins {
        Instruction::Halt => std::process::exit(0),
        Instruction::Set(a, b) => {
            st.regs.set_ir(a, st.regs.get_ir(b));
            *ir += 1;
        }
        Instruction::Push(a) => {
            st.stack.push(st.regs.get_ir(a));
            *ir += 1;
        }
        Instruction::Pop(a) => {
            st.regs.set_ir(a, st.stack.pop().expect("Stack is empty"));
            *ir += 1;
        }
        Instruction::Eq(a, b, c) => {
            st.regs.cmp_op(a, b, c, |x, y| x == y);
            *ir += 1;
        }
        Instruction::Gt(a, b, c) => {
            st.regs.cmp_op(a, b, c, |x, y| x > y);
            *ir += 1;
        }
        Instruction::Jmp(a) => {
            *ir = st.regs.get_ir(a);
        }
        Instruction::Jt(a, b) => {
            if st.regs.get_ir(a) != 0 {
                *ir = st.regs.get_ir(b);
            } else {
                *ir += 1;
            }
        }
        Instruction::Jf(a, b) => {
            if st.regs.get_ir(a) == 0 {
                *ir = st.regs.get_ir(b);
            } else {
                *ir += 1;
            }
        }
        Instruction::Add(a, b, c) => {
            st.regs
                .binary_op(a, b, c, |x, y| ((x as u32 + y as u32) % 32768) as u16);
            *ir += 1;
        }
        Instruction::Mult(a, b, c) => {
            st.regs
                .binary_op(a, b, c, |x, y| ((x as u32 * y as u32) % 32768) as u16);
            *ir += 1;
        }
        Instruction::Mod(a, b, c) => {
            st.regs
                .binary_op(a, b, c, |x, y| ((x as u32 / y as u32) % 32768) as u16);
            *ir += 1;
        }
        Instruction::And(a, b, c) => {
            st.regs.binary_op(a, b, c, |x, y| x & y);
            *ir += 1;
        }
        Instruction::Or(a, b, c) => {
            st.regs.binary_op(a, b, c, |x, y| x | y);
            *ir += 1;
        }
        Instruction::Not(a, b) => {
            st.regs.unary_op(a, b, |x| !x);
            *ir += 1;
        }
        Instruction::RMem(a, b) => {
            st.regs.set_ir(a, st.mem.read(st.regs.get_ir(b)));
            *ir += 1;
        }
        Instruction::WMem(a, b) => {
            st.mem.write(st.regs.get_ir(a), st.regs.get_ir(b));
            *ir += 1;
        }
        Instruction::Call(a) => {
            st.stack.push(*ir + 1);
            *ir = st.regs.get_ir(a);
        }
        Instruction::Ret => {
            let address = st.stack.pop().expect("Stack is empty");
            *ir = address;
        }
        Instruction::Out(a) => {
            print!("{}", st.regs.get_ir(a) as u8 as char);
            *ir += 1;
        }
        Instruction::In(a) => {
            println!("{}: {:?} - NOT IMPLEMENTED", ir, ins);
            *ir += 1;
        }
        Instruction::Noop => *ir += 1,
    }
}

fn main() {
    let bin = load_bin();
    let instructions = instruction::build(&bin);

    let mut storage = Storage::new();
    storage.regs.set_ir(IntReg::Register(RegNb::from(0)), 32766);
    // registers.set(1, 1);

    let mut ir: u16 = 0;
    while ir < instructions.len() as u16 {
        let ins = instructions[ir as usize];
        // println!("{}: {:?}; Regs={:?}", ir, ins, storage.regs);
        execute(&ins, &mut ir, &mut storage);
    }
}
