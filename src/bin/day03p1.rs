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

const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let lines = lines_from_file("in");

    let mut result = 0;
    for line in lines {
        result += score(&line);
    }

    println!("{}", result);
}

fn score(line: &str) -> usize {
    let middle = line.len() / 2;
    for c1 in line[..middle].chars() {
        for c2 in line[middle..].chars() {
            if c1 == c2 {
                return ITEMS.find(c1).unwrap() + 1;
            }
        }
    }

    panic!("no common char");
}
