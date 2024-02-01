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
    run::execute_actions_with_storage(
        &maze::maze_commands::COMMANDS[0..=51],
        &mut ir,
        &mut storage,
        &mut terminal,
    );

    // Patch the program with the correct code and to by-pass the check
    patch_code::patch(&mut storage);

    let msg: String = run::execute_actions_with_storage(
        &maze::maze_commands::COMMANDS[52..],
        &mut ir,
        &mut storage,
        &mut terminal,
    );
    let re =
        Regex::new(r#"But wait!  It looks like someone wrote on your face while you were unconscious on the beach!  Through the mirror, you see \"(\w+)\" scrawled in charcoal on your forehead."#).unwrap();
    let code = re.captures(&msg).unwrap()[1].to_string();
    // Since we saw the code in the mirror, it needs to be flipped :-)
    mirror_code(&code)
}

fn mirror_code(code: &str) -> String {
    // It's both reversing the string, and mirror each char if needed
    code.chars()
        .rev()
        .map(|c| match c {
            'p' => 'q',
            'q' => 'p',
            _ => c,
        })
        .collect()
}

#[test]
fn test_code() {
    assert!(verify_code(7, &code()));
}
