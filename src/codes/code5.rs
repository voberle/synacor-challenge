#![cfg(test)]

use regex::Regex;

use crate::codes::codes_check::verify_code;
use crate::maze;
use crate::vm::run;

fn code() -> String {
    let actions = &maze::maze_commands::COMMANDS[0..=50];

    let msg: String = run::execute_actions(actions);
    let re = Regex::new(r"You activate the teleporter!  As you spiral through time and space, you think you see a pattern in the stars\.\.\.[.\n ]+(\w+)")
        .unwrap();
    re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(5, &code()));
}
