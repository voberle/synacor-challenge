use memoize::memoize;

pub const TELEPORTER_CODE: u16 = 25734;

// To use the teleporter, one has to both by-pass the check and find the correct code.
// To find the correct code, we need to run the below function
// (which actually implements the Ackermann function) on all values between 0 and 32767,
// and use the one that returns 6 (as 6 is the value the challenge checks it against).

// Aka Ackermann function.
// Initially, a = 4 and b = 1, as per the challenge input.
// reg8 is where we are putting the values we are testing (0 to 32767).
//
// By default, this function will overflow its stack and be slow.
// memoize prevents the stack overflow, and makes it a bit faster.
#[memoize]
fn fn6049(a: u16, b: u16, ret8: u16) -> u16 {
    if a == 0 {
        return b + 1;
    }
    if b == 0 {
        return fn6049(a - 1, ret8, ret8);
    }
    fn6049(a - 1, fn6049(a, b - 1, ret8), ret8)
}

// Checks that above function produces the correct answer for our magic value.
pub fn check_fn6049() {
    let ret8 = TELEPORTER_CODE;
    let result = fn6049(4, 1, ret8);
    assert_eq!(result, 6);
}

// Running this functions under tests puts even more pressure on the stack,
// it will only run with:
//     RUST_MIN_STACK=16777216 cargo t --release
// #[test]
// fn test_fn6049() {
//     let result = fn6049(4, 1, 1);
//     assert_eq!(result, 65533);
// }

// To test for all the possible answers, it goes faster if multi-threaded,
// but Rayon itself is recursive, so a bigger stack is needed:
//     RUST_MIN_STACK=8388608 cargo r --release
#[allow(dead_code)]
pub fn find_teleporter_code() {
    use rayon::prelude::*;

    let answer = (0..32767)
        .into_par_iter()
        .find_any(|r| {
            let c = fn6049(4, 1, *r);
            println!("Trying {}: {}", r, c);
            c == 6
        })
        .unwrap();

    println!("Answer {}", answer);
}
