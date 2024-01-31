#![cfg(test)]

// At some point in the maze, there are 5 coins to place in a specific order.
// This piece of code finds the right solution that validates:
// _ + _ * _^2 + _^3 - _ = 399

use itertools::Itertools;

fn find_right_order() -> Vec<&'static str> {
    // There are 5 coins, each with a specific value.
    // You get the value by looking at it: "look red coin".
    const COINS: [(&str, usize); 5] = [
        ("red", 2),
        ("corroded", 3),
        ("shiny", 5),
        ("concave", 7),
        ("blue", 9),
    ];

    COINS
        .iter()
        .permutations(5)
        .find(|p| p[0].1 + p[1].1 * p[2].1.pow(2) + p[3].1.pow(3) - p[4].1 == 399)
        .unwrap()
        .iter()
        .map(|(c, _)| *c)
        .collect()
}

#[test]
fn test_right_order() {
    assert_eq!(
        find_right_order(),
        &["blue", "red", "shiny", "concave", "corroded"]
    );
}
