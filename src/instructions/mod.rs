mod binary_op;
mod cmp_op;
mod halt;
mod jmp;
mod jump_if;
mod mem_access;
mod noop;
mod out;

use std::{fmt::Display, slice::Iter};

use crate::{storage::Storage, terminal::Terminal};

pub trait Instruction: Display {
    fn name(&self) -> &'static str;
    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal);
}

fn unimplemented<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    noop::Noop::inst::<OPCODE>(iter)
}

fn unimplemented_1<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    iter.next();
    noop::Noop::inst::<OPCODE>(iter)
}

fn unimplemented_2<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    iter.next();
    iter.next();
    noop::Noop::inst::<OPCODE>(iter)
}

fn unimplemented_3<const OPCODE: u8>(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    iter.next();
    iter.next();
    iter.next();
    noop::Noop::inst::<OPCODE>(iter)
}

type InstanceFn = fn(&mut Iter<'_, u16>) -> Box<dyn Instruction>;

const BUILDERS: [InstanceFn; 22] = [
    halt::Halt::inst::<0>,
    unimplemented_2::<1>,
    unimplemented_1::<2>,
    unimplemented_1::<3>,
    cmp_op::CmpOp::inst_eq::<4>,
    cmp_op::CmpOp::inst_gt::<5>,
    jmp::Jmp::inst::<6>,
    jump_if::JumpIf::inst_jt::<7>,
    jump_if::JumpIf::inst_jf::<8>,
    binary_op::BinaryOp::inst_add::<9>,
    binary_op::BinaryOp::inst_mult::<10>,
    binary_op::BinaryOp::inst_mod::<11>,
    binary_op::BinaryOp::inst_and::<12>,
    binary_op::BinaryOp::inst_or::<13>,
    unimplemented_2::<14>,
    mem_access::MemAccess::inst_rmem::<15>,
    mem_access::MemAccess::inst_wmem::<16>,
    unimplemented_1::<17>,
    unimplemented::<18>,
    out::Out::inst::<19>,
    unimplemented_1::<20>,
    noop::Noop::inst::<21>,
];

// Build the list of instructions contained in the binary.
pub fn build(bin: &[u16]) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    let mut iter = bin.iter();
    while let Some(opcode) = iter.next() {
        if !(0..=21).contains(opcode) {
            println!("Unknown opcode: {}", opcode);
            // println!("Unknown {}, followed by {}, {}", opcode, *iter.next().unwrap(), *iter.next().unwrap());
            // break;
            continue;
        }
        let ins = BUILDERS[*opcode as usize](&mut iter);
        println!("{}", ins);
        instructions.push(ins);
    }
    instructions
}
