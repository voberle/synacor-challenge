#![cfg(test)]

use regex::Regex;

use crate::codes::codes_check::verify_code;
use crate::maze;
use crate::vm::run;

fn code() -> String {
    let actions = &maze::maze_commands::COMMANDS[0..=15];

    let msg: String = run::execute_actions(actions);
    let re = Regex::new(r"Chiseled on the wall of one of the passageways, you see:[.\n ]+(\w+)")
        .unwrap();
    re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(4, &code()));
}
