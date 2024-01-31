#![cfg(test)]

use regex::Regex;

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
        if ins.name() == "in" {
            break;
        }
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }

    let msg: String = terminal.flush_out();
    let re = Regex::new(r"self-test completion code is: (\w+)").unwrap();
    re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(2, &code()));
}
