use crate::vm::instructions::is_opcode;

use super::instructions::get_instruction;
use super::register::RegNb;
use super::storage::Storage;

// Actions that the debugger may set and that need to be used by the runner.
#[derive(Default)]
pub struct DebuggerActions {
    pub quit: Option<bool>,
    pub verbose: Option<bool>,
    pub set_register: Option<(RegNb, u16)>,
    pub set_memory: Option<(u16, u16)>,
}

// Executes the debugger command.
// This function doesn't modify the state of the program directly, but if it needs to,
// it indicates it via the returned actions.
pub fn exec_debug_cmd(s: &str, ir: u16, storage: &Storage) -> DebuggerActions {
    if s.is_empty() {
        return DebuggerActions {
            ..Default::default()
        };
    }

    let parts: Vec<_> = s.split_whitespace().collect();
    match parts[0] {
        "view" => {
            let n = parts.get(1).unwrap_or(&"1").parse::<u16>().unwrap_or(1);
            show_n_instructions(ir, n, storage);
        }
        "regs" => {
            println!("Registers: {}", storage.regs);
        }
        "stack" => {
            println!("Stack: {:?}", storage.stack);
        }
        "print" => {
            if parts.len() < 2 {
                return DebuggerActions::default();
            }
            if let Ok(address) = parts[1].parse::<u16>() {
                println!("[{}] {}", address, storage.mem.read(address));
            }
        }
        "show" => {
            if parts.len() < 2 {
                return DebuggerActions::default();
            }
            if let Ok(address) = parts[1].parse::<u16>() {
                let n = parts.get(2).unwrap_or(&"1").parse::<u16>().unwrap_or(1);
                show_n_instructions(address, n, storage);
            }
        }
        "verbose" => {
            let verbose = parts.len() > 1 && parts[1] == "on";
            println!("Verbose mode {}", if verbose { "ON" } else { "OFF" });
            return DebuggerActions {
                verbose: Some(true),
                ..Default::default()
            };
        }
        "setr" => {
            if parts.len() < 3 {
                return DebuggerActions::default();
            }
            if let Ok(reg_nb) = parts[1].parse::<usize>() {
                if RegNb::is_valid(reg_nb) {
                    if let Ok(val) = parts[2].parse::<u16>() {
                        return DebuggerActions {
                            set_register: Some((RegNb::new(reg_nb), val)),
                            ..Default::default()
                        };
                    }
                }
            }
        }
        "setm" => {
            if parts.len() < 3 {
                return DebuggerActions::default();
            }
            if let Ok(address) = parts[1].parse::<u16>() {
                if let Ok(val) = parts[2].parse::<u16>() {
                    return DebuggerActions {
                        set_memory: Some((address, val)),
                        ..Default::default()
                    };
                }
            }
        }
        "q" | "quit" => {
            println!("Quitting debugger");
            return DebuggerActions {
                quit: Some(true),
                ..Default::default()
            };
        }
        _ => {
            println!(
                r"Debugger help:

view n      Show next <n> instructions.
regs        Show registers.
stack       Show stack.
print a     Prints value at address <a>.
show a n    Displays <n> instruction at address <a>.
verbose [on|off] Turns verbose mode on/off.
setr r val  Set register <r> to <val>.
setm a val  Set memory address <a> to <val>.
quit        Quit debugger.
"
            );
        }
    }

    DebuggerActions::default()
}

fn show_n_instructions(address: u16, n: u16, storage: &Storage) {
    let mut a = address;
    for _ in 0..n {
        if !is_opcode(storage.mem.read(address)) {
            return;
        }
        let ins = get_instruction(storage, a);
        println!("[{}] {}", a, ins);
        a += ins.offset();
    }
}
