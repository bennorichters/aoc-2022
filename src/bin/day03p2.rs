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
    for i in (0..lines.len()).step_by(3) {
        result += score(&lines[i], &lines[i + 1], &lines[i + 2]);
    }

    println!("{}", result);
}

fn score(line1: &str, line2: &str, line3: &str) -> usize {
    for k in 0..line1.len() {
        let c = line1.chars().nth(k).unwrap();
        if line2.contains(c) && line3.contains(c) {
            return ITEMS.find(c).unwrap() + 1;
        }
    }

    panic!("no common char");
}
