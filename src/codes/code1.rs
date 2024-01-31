#![cfg(test)]

use crate::codes::codes_check::verify_code;
use crate::vm::instructions::get_instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

fn code() -> String {
    let mut ir = 0;
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(false);
    loop {
        let ins = get_instruction(&storage, ir);
        if ins.name() != "out" && ins.name() != "noop" {
            break;
        }
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }

    let msg: String = terminal.flush_out();
    let welcome_re = regex::Regex::new(r"into the challenge website: (\w+)").unwrap();
    welcome_re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(1, &code()));
}
