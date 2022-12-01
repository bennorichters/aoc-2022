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
    let iter = lines.split(|e| e.is_empty());

    let mut all: Vec<i32> = Vec::new();
    for elf in iter {
        let sum = elf.iter().map(|e| e.parse::<i32>().unwrap()).sum();
        all.push(sum);
    }

    all.sort();
    all.reverse();
    println!("{:?}", all[0]);
}
