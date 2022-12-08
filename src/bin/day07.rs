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

    let dirs = dir_sizes(&lines);

    let part1: u32 = dirs.iter().filter(|&e| e < &100_000).sum();

    let total_used: &u32 = dirs.last().unwrap();
    let left_space = 70_000_000 - total_used;
    let to_free = 30_000_000 - left_space;
    let part2: &u32 = dirs.iter().filter(|e| **e >= to_free).min().unwrap();

    println!("part 1: {}, part 2: {}", part1, part2);
}

fn dir_sizes(lines: &[String]) -> Vec<u32> {
    let mut dirs: Vec<u32> = Vec::new();

    let mut to_parse = (&lines[2..]).iter();
    let mut size_stack: Vec<u32> = vec![0];
    while !size_stack.is_empty() {
        let to_parse_next = to_parse.next();
        let mut pop_stack = false;

        if let Some(line) = to_parse_next {
            let s: Vec<&str> = line.split(" ").collect();
            if s[1] == "cd" && s[2] != ".." {
                size_stack.push(0);
            } else if s[1] == "cd" {
                pop_stack = true;
            } else if let Ok(number) = s[0].parse::<u32>() {
                *size_stack.last_mut().unwrap() += number;
            }
        }

        if to_parse_next.is_none() || pop_stack {
            let sub = size_stack.pop().unwrap();
            dirs.push(sub);
            if let Some(last) = size_stack.last_mut() {
                *last += sub;
            }
        }
    }

    dirs
}
