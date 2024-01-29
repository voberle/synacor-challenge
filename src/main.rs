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

const DEBUG: bool = false;

fn main() {
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(!DEBUG);

    let mut ir: u16 = 0;

    loop {
        // println!("\t{}", storage.regs);
        // println!("[{}] {}", ir, ins);
        let ins = get_instruction(&storage, ir);
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }
}
