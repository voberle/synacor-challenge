#![cfg(test)]

use regex::Regex;

use crate::codes::codes_check::verify_code;
use crate::vm::run;

fn code() -> String {
    let actions = ["take tablet", "use tablet"];

    let msg = run::execute_actions(&actions);
    let re = Regex::new(r#"You find yourself writing \"(\w+)\" on the tablet"#).unwrap();
    re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(3, &code()));
}
