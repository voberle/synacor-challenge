use std::fs;

mod codes;
mod instruction;

use instruction::Instruction;
use regex::Regex;

fn load_bin() -> Vec<u16> {
    let bytes = fs::read("resources/challenge.bin").unwrap();
    // Converting to u16 with safe code
    bytes
        .chunks_exact(2)
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .collect()
}

fn code0() {
    // First code was in the spec.
    codes::check_code(0, "LDOb7UGhTi");
}

fn code1(instructions: &[Instruction]) {
    let welcome_msg: String = instructions
        .iter()
        .take_while(|ins| !matches!(ins, Instruction::Halt))
        .flat_map(|ins| {
            if let Instruction::Out(a) = ins {
                Some(*a as u8 as char)
            } else {
                None
            }
        })
        .collect();
    println!("{}", welcome_msg);
    let welcome_re = Regex::new(r"into the challenge website: (\w+)").unwrap();
    codes::check_code(1, &welcome_re.captures(&welcome_msg).unwrap()[1]);
}

fn main() {
    code0();

    let bin = load_bin();
    let instructions = instruction::build(&bin);
    code1(&instructions);
}
