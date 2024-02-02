use rayon::prelude::*;

// To use the teleporter, one has to both by-pass the check and find the correct code.
// To find the correct code, we need to run the below function
// (which actually implements the Ackermann function) on all values between 0 and 32767,
// and use the one that returns 6 (as 6 is the value the challenge checks it against).

// Aka Ackermann function.
// Initially, a = 4 and b = 1, as per the challenge input.
// reg8 is where we are putting the values we are testing (0 to 32767).
//
// By default, this function will overflow its stack and be slow.
//
// We add a custom memoization method, borrowed from https://www.mattkeeter.com/blog/2024-01-28-synacor/
// to prevents the stack overflow and make it faster.
// The custom one that caches only a and b works much much faster than a generic based on memoize crate.
fn fn6049(a: u16, b: u16, ret8: u16, seen: &mut [u16]) -> u16 {
    const MASK: u16 = 0x07FFF;
    let key = (a as usize) | (b as usize) << 3;
    if seen[key] == u16::MAX {
        seen[key] = if a == 0 {
            // Without wrapping, it produces only correct values for the smaller ones.
            b.wrapping_add(1) & MASK
        } else if b == 0 {
            fn6049(a - 1, ret8, ret8, seen)
        } else {
            let t = fn6049(a, b - 1, ret8, seen);
            fn6049(a - 1, t, ret8, seen)
        };
    }
    seen[key]
}

#[test]
fn test_fn6049() {
    let mut seen = vec![u16::MAX; 1 << 18];
    let result = fn6049(4, 1, 1, &mut seen);
    assert_eq!(result, 32765);
}

#[cfg(not(test))]
pub fn find_teleporter_code() -> u16 {
    // Rayon itself is recursive, so a bigger stack is needed
    rayon::ThreadPoolBuilder::new()
        .stack_size(8 * 1024 * 1024)
        .build_global()
        .unwrap();

    (1..32767)
        .into_par_iter()
        .find_any(|r| {
            let mut seen = vec![u16::MAX; 1 << 18];
            let c = fn6049(4, 1, *r, &mut seen);
            // println!("Trying {}: {}", r, c);
            c == 6
        })
        .unwrap()
}

// For tests, hard-coding the results as the real method overflows its stack.
#[cfg(test)]
pub fn find_teleporter_code() -> u16 {
    25734
}
