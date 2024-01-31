use std::collections::VecDeque;
use std::io::{self, Write};

use crate::vm::debugger;
use crate::vm::instructions::get_instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

fn get_next_action(saved_actions: &mut VecDeque<&str>) -> Option<String> {
    if !saved_actions.is_empty() {
        let next_action = saved_actions.pop_front().unwrap();
        Some(format!("{}\n", next_action))
    } else {
        None
    }
}

// Runs the program with the specified actions, returning the terminal output once done.
#[cfg(test)]
pub fn execute_actions(actions: &[&str]) -> String {
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(false);
    let mut ir: u16 = 0;

    let mut saved_actions: VecDeque<&str> = VecDeque::new();
    saved_actions.extend(actions.iter().copied());

    loop {
        let ins = get_instruction(&storage, ir);

        if ins.name() == "in" && terminal.is_input_empty() {
            if let Some(action) = get_next_action(&mut saved_actions) {
                terminal.set_input(&action);
            } else {
                break;
            }
        }

        ins.exec(&mut ir, &mut storage, &mut terminal);
    }

    terminal.flush_out()
}

// Runs the program, first executing the actions, then waiting for user input.
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

        if ins.name() == "in" && terminal.is_input_empty() {
            if let Some(action) = get_next_action(&mut saved_actions) {
                print!("{}", action);
                terminal.set_input(&action);
            }
        }

        ins.exec(&mut ir, &mut storage, &mut terminal);

        while terminal.is_interactive_mode() {
            if interactive_mode(ir, &storage) {
                terminal.quit_interactive_mode();
            }
        }
    }
}

// Returns true if mode should be exited.
fn interactive_mode(ir: u16, storage: &Storage) -> bool {
    print!("> ");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");

    debugger::exec_debug_cmd(buf.trim(), ir, storage)
}
