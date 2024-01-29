mod halt;
mod noop;
mod out;

use std::slice::Iter;

use crate::{storage::Storage, terminal::Terminal};

pub trait Instruction {
    fn name(&self) -> &'static str;
    fn exec(&self, ir: &mut u16, st: &mut Storage, term: &mut Terminal);
}

fn unimplemented(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    noop::Noop::new(iter)
}

fn unimplemented_1(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    iter.next();
    noop::Noop::new(iter)
}

fn unimplemented_2(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    iter.next();
    iter.next();
    noop::Noop::new(iter)
}

fn unimplemented_3(iter: &mut Iter<'_, u16>) -> Box<dyn Instruction> {
    iter.next();
    iter.next();
    iter.next();
    noop::Noop::new(iter)
}

const BUILDERS: [fn(&mut Iter<'_, u16>) -> Box<dyn Instruction>; 22] = [
    halt::Halt::new, // 0
    unimplemented_2, // 1
    unimplemented_1, // 2
    unimplemented_1, // 3
    unimplemented_3, // 4
    unimplemented_3, // 5
    unimplemented_1, // 6
    unimplemented_2, // 7
    unimplemented_2, // 8
    unimplemented_3, // 9
    unimplemented_3, // 10
    unimplemented_3, // 11
    unimplemented_3, // 12
    unimplemented_3, // 13
    unimplemented_2, // 14
    unimplemented_2, // 15
    unimplemented_2, // 16
    unimplemented_1, // 17
    unimplemented,   // 18
    out::Out::new,   // 19
    unimplemented_1, // 20
    noop::Noop::new, // 21
];

pub fn build(bin: &[u16]) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    let mut iter = bin.iter();
    while let Some(opcode) = iter.next() {
        // println!("{}", opcode);
        if !(0..=21).contains(opcode) {
            break;
        }
        instructions.push(BUILDERS[*opcode as usize](&mut iter))
    }
    instructions
}
