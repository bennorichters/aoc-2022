#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|e| e.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(i32, i32, i32);

fn solve() {
    let lines = lines_from_file("in");

    let mut cubes: Vec<Coord> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();
        cubes.push(Coord(
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
            parts[2].parse::<i32>().unwrap(),
        ));
    }

    walk(cubes);
}

fn walk(cubes: Vec<Coord>) {
    let mut state_stack: Vec<Coord> = Vec::new();
    state_stack.push(Coord(0, 0, 0));

    let mut visited: HashSet<Coord> = HashSet::new();

    let mut shared = 0;
    while !state_stack.is_empty() {
        let state = state_stack.pop().unwrap();
        if cubes.contains(&state) || !visited.insert(state.clone()) {
            continue;
        }

        shared += cubes.iter().filter(|c| shares_side(&state, &c)).count();

        if state.0 >= 0 {
            state_stack.push(Coord(state.0 - 1, state.1, state.2));
        }
        if state.0 < 24 {
            state_stack.push(Coord(state.0 + 1, state.1, state.2));
        }
        if state.1 >= 0 {
            state_stack.push(Coord(state.0, state.1 - 1, state.2));
        }
        if state.1 < 24 {
            state_stack.push(Coord(state.0, state.1 + 1, state.2));
        }
        if state.2 >= 0 {
            state_stack.push(Coord(state.0, state.1, state.2 - 1));
        }
        if state.2 < 24 {
            state_stack.push(Coord(state.0, state.1, state.2 + 1));
        }
    }

    println!("{}", shared);
}

fn shares_side(a: &Coord, b: &Coord) -> bool {
    let mut equal = 0;
    let mut one_diff = 0;

    match (a.0 - b.0).abs() {
        0 => equal += 1,
        1 => one_diff += 1,
        _ => {}
    }
    match (a.1 - b.1).abs() {
        0 => equal += 1,
        1 => one_diff += 1,
        _ => {}
    }
    match (a.2 - b.2).abs() {
        0 => equal += 1,
        1 => one_diff += 1,
        _ => {}
    }

    equal == 2 && one_diff == 1
}
