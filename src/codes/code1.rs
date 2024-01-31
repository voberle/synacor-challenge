#![cfg(test)]

use regex::Regex;

use crate::codes::codes_check::verify_code;
use crate::vm::run;

fn code() -> String {
    let msg = run::execute_actions(&[]);
    let re = Regex::new(r"into the challenge website: (\w+)").unwrap();
    re.captures(&msg).unwrap()[1].to_string()
}

#[test]
fn test_code() {
    assert!(verify_code(1, &code()));
}
