use crate::vm::instructions::is_opcode;

use super::instructions::get_instruction;
use super::storage::Storage;

// Executes the debugger command.
// Returns true if debugger should exit.
pub fn exec_debug_cmd(s: &str, ir: u16, storage: &Storage) -> bool {
    let parts: Vec<_> = s.split_whitespace().collect();
    match parts[0] {
        "view" => {
            if let Ok(n) = parts[1].parse::<u16>() {
                let mut t = ir;
                for i in 0..n {
                    let ins = get_instruction(storage, t);
                    println!("[{}] {}", ir + i, ins);
                    t += ins.offset();
                }
            }
        }
        "regs" => {
            println!("Registers: {}", storage.regs);
        }
        "stack" => {
            println!("Stack: {:?}", storage.stack);
        }
        "print" => {
            if let Ok(address) = parts[1].parse::<u16>() {
                println!("[{}] {}", address, storage.mem.read(address));
            }
        }
        "show" => {
            if let Ok(address) = parts[1].parse::<u16>() {
                if is_opcode(storage.mem.read(address)) {
                    let ins = get_instruction(storage, address);
                    println!("[{}] = {}", address, ins);
                }
            }
        }
        "q" | "quit" => {
            println!("Quitting debugger");
            return true;
        }
        _ => {
            println!(
                r"Debugger help:
view n  Show next n instructions.
regs    Show registers.
stack   Show stack.
print a Prints value at address <a>.
show a  Displays instruction at address <a>.
quit    Quit debugger.
"
            );
        }
    }
    false
}
