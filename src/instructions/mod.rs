mod binary_op;
mod call;
mod cmp_op;
mod halt;
mod input;
mod jmp;
mod jump_if;
mod mem_access;
mod noop;
mod out;
mod ret;
mod set;
mod stack;
mod unary_op;

use std::fmt::Display;

use crate::{storage::Storage, terminal::Terminal};

pub trait Instruction: Display {
    fn name(&self) -> &'static str;

    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal);
}

type InstanceFn = fn(&Storage, u16) -> Box<dyn Instruction>;

const BUILDERS: [InstanceFn; 22] = [
    halt::Halt::inst::<0>,
    set::Set::inst::<1>,
    stack::Stack::inst_push::<2>,
    stack::Stack::inst_pop::<3>,
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
    unary_op::Not::inst::<14>,
    mem_access::MemAccess::inst_rmem::<15>,
    mem_access::MemAccess::inst_wmem::<16>,
    call::Call::inst::<17>,
    ret::Ret::inst::<18>,
    out::Out::inst::<19>,
    input::In::inst::<20>,
    noop::Noop::inst::<21>,
];

pub fn get_instruction(storage: &Storage, address: u16) -> Box<dyn Instruction> {
    let opcode = storage.mem.read(address);
    // println!("opcode={opcode}");
    assert!((0..=21).contains(&opcode));
    BUILDERS[opcode as usize](storage, address)
}
