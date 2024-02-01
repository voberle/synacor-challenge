#![cfg(test)]

use regex::Regex;

use crate::codes::codes_check::verify_code;
use crate::maze;
use crate::maze::patch_code;
use crate::vm::run;
use crate::vm::storage::Storage;
use crate::vm::terminal::Terminal;

fn code() -> String {
    let mut storage = Storage::new();
    let mut ir: u16 = 0;
    let mut terminal = Terminal::new(false);

    // Run first set of actions until we find the teleporter
    let actions = &maze::maze_commands::COMMANDS[0..=51];
    run::execute_actions_with_storage(actions, &mut ir, &mut storage, &mut terminal);

    // Patch the program with the correct code and to by-pass the check
    patch_code::patch(&mut storage);

    let msg: String = run::execute_actions_with_storage(
        &["use teleporter"],
        &mut ir,
        &mut storage,
        &mut terminal,
    );
    let re =
        Regex::new(r"Someone seems to have drawn a message in the sand here:[.\n ]+(\w+)").unwrap();
    re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(6, &code()));
}
