mod binary;
mod codes;
mod instructions;
mod intreg;
mod register;
mod storage;
mod terminal;

use binary::load_bin;
use instructions::build;
use register::RegNb;
use storage::Storage;
use terminal::Terminal;

const DEBUG: bool = true;

fn main() {
    let bin = load_bin();
    let instructions = build(&bin);

    let mut storage = Storage::new();
    let mut terminal = Terminal::new(!DEBUG);
    // Something happens if we set register 0 to this value
    storage.regs.set(RegNb::new(0), 32766);

    let mut ir: u16 = 0;
    while ir < instructions.len() as u16 {
        let ins = &instructions[ir as usize];
        if DEBUG {
            println!("[{}] {};\tRegs={}", ir, ins, storage.regs);
        }
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }
}
