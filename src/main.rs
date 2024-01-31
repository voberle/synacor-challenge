mod codes;
mod maze;
mod vm;

use std::collections::VecDeque;

use vm::instructions::get_instruction;
use vm::storage::Storage;
use vm::terminal::Terminal;

const DEBUG: bool = false;

fn main() {
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(!DEBUG);
    let mut ir: u16 = 0;

    let mut saved_actions = VecDeque::from(maze::maze_commands::COMMANDS);

    let mut i = 0;
    loop {
        let ins = get_instruction(&storage, ir);
        if DEBUG {
            // println!("\t{}", storage.regs);
            println!("{i}: [{}] {}", ir, ins);
            i += 1;
        }
        if ins.name() == "in" && terminal.is_input_empty() {
            if !saved_actions.is_empty() {
                let next_action = saved_actions.pop_front().unwrap();
                let cmd = format!("{}\n", next_action);
                terminal.set_input(&cmd);
            }
        }

        ins.exec(&mut ir, &mut storage, &mut terminal);
    }
}
