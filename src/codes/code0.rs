#![cfg(test)]

use crate::codes::codes_check::verify_code;

fn code0() -> String {
    // First code was in the spec.
    "LDOb7UGhTi".to_string()
}

#[test]
fn test_code0() {
    assert!(verify_code(0, &code0()));
}
