#![cfg(test)]

use crate::codes::codes_check::verify_code;
use crate::vm::instructions::get_instruction;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

fn code() -> String {
    let mut ir = 0;
    let mut storage = Storage::new();
    let mut terminal = Terminal::new(false);
    terminal.set_input(
        r"take tablet
use tablet
",
    );
    for _ in 0..703217 {
        let ins = get_instruction(&storage, ir);
        ins.exec(&mut ir, &mut storage, &mut terminal);
    }

    let msg: String = terminal.flush_out();
    let welcome_re =
        regex::Regex::new(r#"You find yourself writing \"(\w+)\" on the tablet"#).unwrap();
    welcome_re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(3, &code()));
}
