#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
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

    let mut result = 0;
    for i in 0..cubes.len() {
        let a = &cubes[i];
        let mut open: i32 = 6;
        for j in 0..cubes.len() {
            if i != j {
                let b = &cubes[j];

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

                if equal == 2 && one_diff == 1 {
                    open -= 1;
                }
            }
        }

        result += open;
    }

    println!("{}", result);
}
