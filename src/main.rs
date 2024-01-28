use std::fs;

mod codes;
mod instruction;

use instruction::Instruction;
use regex::Regex;

use crate::instruction::Number;

fn load_bin() -> Vec<u16> {
    let bytes = fs::read("resources/challenge.bin").unwrap();
    // Converting to u16 with safe code
    bytes
        .chunks_exact(2)
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .collect()
}

#[cfg(test)]
fn code0() -> String {
    // First code was in the spec.
    "LDOb7UGhTi".to_string()
}

fn code1(instructions: &[Instruction]) -> String {
    let welcome_msg: String = instructions
        .iter()
        .take_while(|ins| !matches!(ins, Instruction::Halt))
        .flat_map(|ins| {
            if let Instruction::Out(a) = ins {
                if let Number::Value(v) = a {
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
    let welcome_re = Regex::new(r"into the challenge website: (\w+)").unwrap();
    welcome_re.captures(&welcome_msg).unwrap()[1].to_string()
}

fn main() {
    let bin = load_bin();
    let instructions = instruction::build(&bin);
    let code = code1(&instructions);
    codes::check_code(1, &code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code0() {
        assert!(codes::verify_code(0, &code0()));
    }

    #[test]
    fn test_code1() {
        let bin = load_bin();
        let instructions = instruction::build(&bin);
        assert!(codes::verify_code(1, &code1(&instructions)));
    }
}
