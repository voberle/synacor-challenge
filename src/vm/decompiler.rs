//! Tool to decompile the binary, to help with reverse-engineering it.

// Format:
// 2022   eq  reg[4]  reg[2]   10000

use crate::vm::instructions::BUILDERS;

use super::instructions::is_opcode;
use super::storage::Memory;

#[allow(dead_code)]
pub fn decompile() {
    let mem = Memory::new();

    let mut address = 0;
    while address < mem.len() {
        let opcode = mem.read(address);
        // All the code seems to be before 6090
        if address < 6090 && is_opcode(opcode) {
            let ins = BUILDERS[opcode as usize](address, mem.ins_slice(address));
            println!("{}", ins.decompile());
            address += ins.offset();
        } else {
            // TODO add instruction validation to the instruction instance methods.

            // if is_opcode(opcode) {
            //     println!("Possible opcode: {}\t{}", address, mem.read(address));
            // }
            println!("{}\t{}", address, mem.read(address));
            address += 1;
        }
    }
}
