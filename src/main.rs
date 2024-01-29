mod binary;
mod codes;
mod instructions;
mod intreg;
mod register;
mod storage;
mod terminal;

use binary::load_bin;
use instructions::build;
use intreg::IntReg;
use register::RegNb;
use storage::Storage;

fn main() {
    let bin = load_bin();
    let instructions = build(&bin);

    let mut storage = Storage::new();
    storage.regs.set_ir(IntReg::Register(RegNb::from(0)), 32766);
    // registers.set(1, 1);

    // let mut ir: u16 = 0;
    // while ir < instructions.len() as u16 {
    //     let ins = instructions[ir as usize];
    //     // println!("{}: {:?}; Regs={:?}", ir, ins, storage.regs);
    //     // execute(&ins, &mut ir, &mut storage);
    // }
}
