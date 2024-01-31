use std::collections::VecDeque;

use crate::vm::instructions::get_instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

fn enter_next_action(terminal: &mut Terminal, saved_actions: &mut VecDeque<&str>) {
    if terminal.is_input_empty() && !saved_actions.is_empty() {
        let next_action = saved_actions.pop_front().unwrap();
        let cmd = format!("{}\n", next_action);
        terminal.set_input(&cmd);
    }
}

pub fn execute_program(actions: &[&str], debug: bool) {
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(!debug);
    let mut ir: u16 = 0;

    let mut saved_actions: VecDeque<&str> = VecDeque::new();
    saved_actions.extend(actions.iter().copied());

    let mut i = 0;
    loop {
        let ins = get_instruction(&storage, ir);
        if debug {
            // println!("\t{}", storage.regs);
            println!("{i}: [{}] {}", ir, ins);
            i += 1;
        }
        if ins.name() == "in" {
            enter_next_action(&mut terminal, &mut saved_actions);
        }

        ins.exec(&mut ir, &mut storage, &mut terminal);
    }
}
