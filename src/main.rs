use std::fs;

mod codes;
mod instruction;
mod storage;

use instruction::{Instruction, IntReg};
use storage::{Registers, Storage};

fn load_bin() -> Vec<u16> {
    let bytes = fs::read("resources/challenge.bin").unwrap();
    // Converting to u16 with safe code
    bytes
        .chunks_exact(2)
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .collect()
}

#[cfg(test)]
fn code0() -> String {
    // First code was in the spec.
    "LDOb7UGhTi".to_string()
}

#[cfg(test)]
fn code1(instructions: &[Instruction]) -> String {
    use crate::instruction::IntReg;

    let welcome_msg: String = instructions
        .iter()
        .take_while(|ins| !matches!(ins, Instruction::Halt))
        .flat_map(|ins| {
            if let Instruction::Out(a) = ins {
                if let IntReg::Value(v) = a {
                    Some(*v as u8 as char)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    println!("{}", welcome_msg);
    let welcome_re = regex::Regex::new(r"into the challenge website: (\w+)").unwrap();
    welcome_re.captures(&welcome_msg).unwrap()[1].to_string()
}

fn execute(ins: &Instruction, ir: &mut u16, regs: &mut Registers, stack: &mut Vec<u16>) {
    match *ins {
        Instruction::Halt => std::process::exit(0),
        Instruction::Set(a, b) => {
            regs.set_ir(a, regs.get_ir(b));
            *ir += 1;
        }
        Instruction::Push(a) => {
            stack.push(regs.get_ir(a));
            *ir += 1;
        }
        Instruction::Pop(a) => {
            regs.set(regs.get_ir(a), stack.pop().expect("Stack is empty"));
            *ir += 1;
        }
        Instruction::Eq(a, b, c) => {
            regs.cmp_op(a, b, c, |x, y| x == y);
            *ir += 1;
        }
        Instruction::Gt(a, b, c) => {
            regs.cmp_op(a, b, c, |x, y| x > y);
            *ir += 1;
        }
        Instruction::Jmp(a) => {
            *ir = regs.get_ir(a);
        }
        Instruction::Jt(a, b) => {
            if regs.get_ir(a) != 0 {
                *ir = regs.get_ir(b);
            } else {
                *ir += 1;
            }
        }
        Instruction::Jf(a, b) => {
            if regs.get_ir(a) == 0 {
                *ir = regs.get_ir(b);
            } else {
                *ir += 1;
            }
        }
        Instruction::Add(a, b, c) => {
            regs.binary_op(a, b, c, |x, y| ((x as u32 + y as u32) % 32768) as u16);
            *ir += 1;
        }
        Instruction::Mult(a, b, c) => {
            regs.binary_op(a, b, c, |x, y| ((x as u32 * y as u32) % 32768) as u16);
            *ir += 1;
        }
        Instruction::Mod(a, b, c) => {
            regs.binary_op(a, b, c, |x, y| ((x as u32 / y as u32) % 32768) as u16);
            *ir += 1;
        }
        Instruction::And(a, b, c) => {
            regs.binary_op(a, b, c, |x, y| x & y);
            *ir += 1;
        }
        Instruction::Or(a, b, c) => {
            regs.binary_op(a, b, c, |x, y| x | y);
            *ir += 1;
        }
        Instruction::Not(a, b) => {
            regs.unary_op(a, b, |x| !x);
            *ir += 1;
        }
        Instruction::RMem(a, b) => {
            println!("{}: {:?} - NOT IMPLEMENTED", ir, ins);
            *ir += 1;
        }
        Instruction::WMem(a, b) => {
            println!("{}: {:?} - NOT IMPLEMENTED", ir, ins);
            *ir += 1;
        }
        Instruction::Call(a) => {
            stack.push(*ir + 1);
            *ir = regs.get_ir(a);
        }
        Instruction::Ret => {
            println!("{}: {:?} - NOT IMPLEMENTED", ir, ins);
            *ir += 1;
        }
        Instruction::Out(a) => {
            print!("{}", regs.get_ir(a) as u8 as char);
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
    storage.regs.set(0, 32766);
    // registers.set(1, 1);

    let mut ir: u16 = 0;
    while ir < instructions.len() as u16 {
        let ins = instructions[ir as usize];
        println!("{}: {:?}; Regs={:?}", ir, ins, storage.regs);
        execute(&ins, &mut ir, &mut storage.regs, &mut storage.stack);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code0() {
        assert!(codes::verify_code(0, &code0()));
    }

    #[test]
    fn test_code1() {
        let bin = load_bin();
        let instructions = instruction::build(&bin);
        assert!(codes::verify_code(1, &code1(&instructions)));
    }
}
