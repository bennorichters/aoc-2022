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

fn solve() {
    let lines = lines_from_file("in");

    let mut result =0;
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut int_cyc = 20;
    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();

        let prev = x;
        if s[0] == "addx" {
            let nr = s[1].parse::<i32>().unwrap();
            x += nr;
            cycle += 2;
        } else {
            cycle += 1;
        }

        if cycle >= int_cyc {
            println!("{}, {}", cycle, prev);
            result += int_cyc * prev;
            int_cyc += 40;
        }
    }

    println!("{:?}", result);
}
