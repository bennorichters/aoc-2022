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
    let m = line.len() / 2;
    for i in 0..m {
        let c1 = line.chars().nth(i).unwrap();
        for j in m..(m * 2) {
            let c2 = line.chars().nth(j).unwrap();
            if c1 == c2 {
                return ITEMS.find(c1).unwrap() + 1;
            }
        }
    }

    panic!("no common char");
}
