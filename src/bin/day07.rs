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

    let dirs = process_dir(&lines);

    let part1: u32 = dirs.iter().filter(|&e| e < &100_000).sum();

    let total_used: &u32 = dirs.last().unwrap();
    let left_space = 70_000_000 - total_used;
    let to_free = 30_000_000 - left_space;
    let part2: &u32 = dirs.iter().filter(|e| **e >= to_free).min().unwrap();

    println!("part 1: {}, part 2: {}", part1, part2);
}

fn process_dir(lines: &[String]) -> Vec<u32> {
    let mut dirs: Vec<u32> = Vec::new();

    let mut stack_size: Vec<u32> = Vec::new();
    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();

        if s[1] == "cd" && s[2] != ".." {
            stack_size.push(0);
        } else if s[1] == "cd" {
            let sub = stack_size.pop().unwrap();
            dirs.push(sub);
            *stack_size.last_mut().unwrap() += sub;
        } else {
            let number = s[0].parse::<u32>();
            if number.is_ok() {
                *stack_size.last_mut().unwrap() += number.unwrap();
            }
        }
    }

    let mut total = 0;
    for sub in stack_size.iter().rev() {
        total += sub;
        dirs.push(total);
    }

    dirs
}
