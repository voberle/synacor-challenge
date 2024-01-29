#![cfg(test)]

use crate::binary::load_bin;
use crate::codes::codes_check::verify_code;
use crate::instructions::{build, Instruction};
use crate::storage::Storage;
use crate::terminal::Terminal;

fn code1(instructions: &[Box<dyn Instruction>]) -> String {
    let mut ir = 0;
    let mut storage = Storage::new();
    let mut term = Terminal::new(false);
    instructions
        .iter()
        .take_while(|ins| ins.name() != "halt")
        .filter(|ins| ins.name() == "out")
        .for_each(|ins| ins.exec(&mut ir, &mut storage, &mut term));
    let welcome_msg: String = term.flush_out();
    println!("{}", welcome_msg);
    let welcome_re = regex::Regex::new(r"into the challenge website: (\w+)").unwrap();
    welcome_re.captures(&welcome_msg).unwrap()[1].to_string()
}

#[test]
fn test_code1() {
    let bin = load_bin();
    let instructions = build(&bin);
    assert!(verify_code(1, &code1(&instructions)));
}
