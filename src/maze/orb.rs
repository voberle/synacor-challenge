//! Code to solve the following problem:
//! Find the shortest path from bottom-left to top-right that results in a sum of 30.
//!
//! *    8   -   1
//! 4    *   11  *
//! +    4   -   18
//! 22   -   9   *

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Room {
    Value(i32),
    Plus,
    Minus,
    Mult,
}
use itertools::Itertools;
use Room::*;

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value(v) => write!(f, "{}", v),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Mult => write!(f, "*"),
        }
    }
}

// Position in the graph, room, list of connected positions.
// We need the position in the graph as some rooms have the same content.
type Graph = Vec<(usize, Room, Vec<usize>)>;

#[allow(dead_code)]
pub fn solve_orb() -> Vec<usize> {
    // From left-right, top-up: Nodes and their connections.
    // 12   13  14  15
    // 8    9   10  11
    // 4    5   6   7
    // 0    1   2   3
    #[rustfmt::skip]
    let graph: Graph = vec![
        (0,  Value(22), vec![1, 4]),
        (1,  Minus,     vec![0, 5, 2]),
        (2,  Value(9),  vec![1, 6, 3]),
        (3,  Mult,      vec![2, 7]),
        (4,  Plus,      vec![0, 5, 8]),
        (5,  Value(4),  vec![1, 6, 9, 4]),
        (6,  Minus,     vec![2, 5, 10, 7]),
        (7,  Value(18), vec![3, 6, 11]),
        (8,  Value(4),  vec![4, 9, 12]),
        (9,  Mult,      vec![8, 5, 10, 13]),
        (10, Value(11), vec![6, 9, 14, 11]),
        (11, Mult,      vec![7, 10, 15]),
        (12, Mult,      vec![8, 13]),
        (13, Value(8),  vec![12, 9, 14]),
        (14, Minus,     vec![13, 10, 15]),
        (15, Value(1),  vec![14, 11]),
    ];

    let mut shortest_found_path: Vec<usize> = Vec::new();
    traverse_graph(&graph, 0, &[0], &mut shortest_found_path);
    println!(
        "Length={}, path: {}",
        shortest_found_path.len(),
        positions_to_str(&graph, &shortest_found_path)
    );

    shortest_found_path
}

#[test]
fn test_solve_orb() {
    let path = solve_orb();
    assert_eq!(path.len(), 13);
}

// Calculates the weight of the list of rooms.
fn calc_weight(rooms: &[Room]) -> i32 {
    assert!(!rooms.is_empty());
    let initial_val = if let Value(v) = rooms[0] {
        v
    } else {
        panic!("Path doesn't start with a value");
    };
    if rooms.len() == 1 {
        return initial_val;
    }

    rooms[1..].chunks(2).fold(initial_val, |acc, r| {
        if r.len() < 2 {
            return acc;
        }
        if let Value(val) = r[1] {
            match r[0] {
                Plus => acc + val,
                Minus => acc - val,
                Mult => acc * val,
                Value(_) => panic!("Wrong path, got value instead of sign"),
            }
        } else {
            panic!("Wrong path, got sign instead of value");
        }
    })
}

#[test]
fn test_calc() {
    assert_eq!(calc_weight(&[Value(22)]), 22);
    assert_eq!(calc_weight(&[Value(22), Plus]), 22);
    assert_eq!(calc_weight(&[Value(22), Plus, Value(13)]), 35);
    assert_eq!(calc_weight(&[Value(22), Plus, Value(13), Minus]), 35);
    assert_eq!(
        calc_weight(&[Value(22), Plus, Value(13), Minus, Value(30)]),
        5
    );
}

// Converts the list of positions forming a path to some nicer string.
fn positions_to_str(graph: &Graph, positions: &[usize]) -> String {
    positions
        .iter()
        .map(|p| {
            let e = &graph[*p];
            format!("{} ({})", e.0, e.1)
        })
        .join(" ")
}

fn positions_to_room_list(graph: &Graph, positions: &[usize]) -> Vec<Room> {
    positions.iter().map(|p| graph[*p].1).collect()
}

// The function that finds all valid path.
// Graph traversal, with DFS, recursive.
fn traverse_graph(graph: &Graph, pos: usize, path: &[usize], shortest_found_path: &mut Vec<usize>) {
    const START_POS: usize = 0;
    const END_POS: usize = 15;
    const END_WEIGHT: i32 = 30;

    let rooms = positions_to_room_list(graph, path);
    let w = calc_weight(&rooms);

    // Weight cannot be negative
    if w < 0 {
        return;
    }

    if pos == END_POS {
        if w == END_WEIGHT {
            // println!("Found {}", positions_to_str(&graph, &path));
            if shortest_found_path.is_empty() || shortest_found_path.len() > path.len() {
                *shortest_found_path = path.to_vec();
            }
        }
        // Once in the Vault room, we cannot leave anymore
        return;
    }

    // If we are on a path bigger than the shortest already found, no need to pursue
    if !shortest_found_path.is_empty() && path.len() > shortest_found_path.len() {
        return;
    }

    // Avoid going too long
    if path.len() > 16 {
        return;
    }

    for neighbor_pos in &graph[pos].2 {
        // We cannot go back to start room anymore, as this causes orb to vanish.
        if *neighbor_pos == START_POS {
            continue;
        }

        let mut new_path = path.to_vec();
        new_path.push(*neighbor_pos);

        traverse_graph(graph, *neighbor_pos, &new_path, shortest_found_path);
    }
}
