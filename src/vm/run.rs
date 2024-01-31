use std::collections::VecDeque;
use std::io::{self, Write};

use crate::vm::debugger;
use crate::vm::instructions::get_instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

use super::debugger::DebuggerActions;

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
pub fn execute_program(actions: &[&str]) {
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(true);
    let mut ir: u16 = 0;

    let mut saved_actions: VecDeque<&str> = VecDeque::new();
    saved_actions.extend(actions.iter().copied());

    let mut verbose = false;
    let mut set_breakpoint: Option<u16> = None;

    loop {
        let ins = get_instruction(&storage, ir);
        if verbose {
            println!("[{}] {}", ir, ins);
        }

        if ins.name() == "in" && terminal.is_input_empty() {
            if let Some(action) = get_next_action(&mut saved_actions) {
                print!("{}", action);
                terminal.set_input(&action);
            }
        }

        if let Some(breakpoint) = set_breakpoint {
            if breakpoint == ir {
                terminal.set_interactive_mode();
                println!("Stopped at breakpoint {}", breakpoint);
            }
        }
        if !terminal.is_interactive_mode() {
            ins.exec(&mut ir, &mut storage, &mut terminal);
        }

        while terminal.is_interactive_mode() {
            let debugger_actions = interactive_mode(ir, &storage);
            if let Some(true) = debugger_actions.quit {
                terminal.quit_interactive_mode();
            }
            if let Some(is_verbose) = debugger_actions.verbose {
                verbose = is_verbose;
            }
            set_breakpoint = debugger_actions.set_breakpoint;
            if let Some(true) = debugger_actions.clear_breakpoint {
                set_breakpoint = None;
            }
            if let Some((reg_nb, val)) = debugger_actions.set_register {
                storage.regs.set(reg_nb, val);
                println!("Register {} set to {}", reg_nb, val);
            }
            if let Some((a, val)) = debugger_actions.set_memory {
                storage.mem.write(a, val);
                println!("Memory at {} set to {}", a, val);
            }
        }
    }
}

fn interactive_mode(ir: u16, storage: &Storage) -> DebuggerActions {
    print!("> ");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");

    debugger::exec_debug_cmd(buf.trim(), ir, storage)
}
