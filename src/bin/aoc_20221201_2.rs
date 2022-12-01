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

    let mut iter = lines.split(|e| e.is_empty());

    let mut all: Vec<i32> = Vec::new();
    loop {
        let elf = iter.next();
        if elf.is_none() {
            break;
        }

        let mut total = 0;
        for c in elf.unwrap() {
            total += c.parse::<i32>().unwrap();
        }
        all.push(total);
    }

    all.sort();
    all.reverse();
    let result = all[0] + all[1] + all[2];
    println!("{:?}", result);
}
