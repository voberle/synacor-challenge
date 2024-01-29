#![cfg(test)]

use crate::codes::codes_check::verify_code;
use crate::instructions::get_instruction;
use crate::storage::Storage;
use crate::terminal::Terminal;

fn code1() -> String {
    let mut ir = 0;
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(false);
    loop {
        let ins = get_instruction(&storage, ir);
        if ins.name() == "halt" {
            break;
        }
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }

    let welcome_msg: String = terminal.flush_out();
    println!("{}", welcome_msg);
    let welcome_re = regex::Regex::new(r"into the challenge website: (\w+)").unwrap();
    welcome_re.captures(&welcome_msg).unwrap()[1].to_string()
}

#[test]
fn test_code1() {
    assert!(verify_code(1, &code1()));
}
