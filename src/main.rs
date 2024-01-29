mod binary;
mod codes;
mod instructions;
mod intreg;
mod register;
mod storage;
mod terminal;

use binary::load_bin;
use instructions::build;
use storage::Storage;
use terminal::Terminal;

const DEBUG: bool = false;

fn main() {
    let bin = load_bin();
    let instructions = build(&bin);

    let mut storage = Storage::new();
    let mut terminal = Terminal::new(!DEBUG);
    // storage.regs.set_ir(IntReg::Register(RegNb::from(0)), 32766);

    let mut ir: u16 = 0;
    while ir < instructions.len() as u16 {
        let ins = &instructions[ir as usize];
        if DEBUG {
            println!("{}: {}; Regs={}", ir, ins, storage.regs);
        }
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }
}
