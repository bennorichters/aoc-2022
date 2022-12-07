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
        .map(|e| e.expect("could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_file("in");
    let mut dirs = Vec::new();
    process_dir(&lines, 1, &mut dirs);

    let total_used = dirs.iter().max().unwrap();
    let left_space = 70_000_000 - total_used;
    let to_free = 30_000_000 - left_space;

    let result: &u32 = dirs.iter().filter(|e| **e >= to_free).min().unwrap();
    println!("{}", result);
}

fn process_dir(lines: &[String], start_index: usize, dirs: &mut Vec<u32>) -> (u32, usize) {
    let mut size: u32 = 0;
    let mut i = start_index + 1;
    while i < lines.len() && lines[i] != "$ cd .." {
        let line = &lines[i];
        let s: Vec<&str> = line.split(" ").collect();

        if s[0] == "$" {
            let sub = process_dir(&lines, i + 1, dirs);
            size += sub.0;
            i = sub.1;
        } else {
            let number = s[0].parse::<u32>();
            if number.is_ok() {
                size += number.unwrap();
            }
        }

        i += 1;
    }

    dirs.push(size);

    (size, i)
}

