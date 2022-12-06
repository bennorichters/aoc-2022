use std::{
    collections::HashSet,
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

    let line = &lines[0];
    let marker = 4;
    for i in 0..(line.len() - marker) {
        let distinct: HashSet<char> = line[i..(i + marker)].chars().collect();
        if distinct.len() == marker {
            println!("{}", i + marker);
            break;
        }
    }
}
