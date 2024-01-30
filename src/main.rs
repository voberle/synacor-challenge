mod binary;
mod codes;
mod instructions;
mod intreg;
mod register;
mod storage;
mod terminal;

use instructions::get_instruction;
use storage::Storage;
use terminal::Terminal;

const DEBUG: bool = true;

fn main() {
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(!DEBUG);

    let mut ir: u16 = 0;

    let mut i = 0;
    loop {
        let ins = get_instruction(&storage, ir);
        if DEBUG {
            // println!("\t{}", storage.regs);
            println!("{i}: [{}] {}", ir, ins);
            i += 1;
        }
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }
}
