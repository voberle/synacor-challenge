#![cfg(test)]

use crate::binary::load_bin;
use crate::codes::codes_check::verify_code;
use crate::instruction::{self, Instruction, IntReg};

fn code1(instructions: &[Instruction]) -> String {
    let welcome_msg: String = instructions
        .iter()
        .take_while(|ins| !matches!(ins, Instruction::Halt))
        .flat_map(|ins| {
            if let Instruction::Out(a) = ins {
                if let IntReg::Value(v) = a {
                    Some(*v as u8 as char)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    println!("{}", welcome_msg);
    let welcome_re = regex::Regex::new(r"into the challenge website: (\w+)").unwrap();
    welcome_re.captures(&welcome_msg).unwrap()[1].to_string()
}

#[test]
fn test_code1() {
    let bin = load_bin();
    let instructions = instruction::build(&bin);
    assert!(verify_code(1, &code1(&instructions)));
}
