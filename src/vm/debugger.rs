use super::instructions::get_instruction;
use super::storage::Storage;

// Executes the debugger command.
// Returns true if debugger should exit.
pub fn exec_debug_cmd(cmd: &str, ir: u16, storage: &Storage) -> bool {
    match cmd {
        "v1" => println!("[1] {}", get_instruction(storage, ir)),
        "v10" => {
            let mut t = ir;
            for i in 0..10 {
                let ins = get_instruction(storage, t);
                println!("[{}] {}", i, ins);
                t += ins.offset();
            }
        }
        _ => {
            println!("Quitting debugger");
            return true;
        }
    }
    false
}
