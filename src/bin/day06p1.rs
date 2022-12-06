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

    let marker = 4;
    let a = (&lines[0].as_bytes())
        .windows(marker)
        .map(|w| w.iter().collect::<HashSet<_>>())
        .position(|w| w.len() == marker)
        .unwrap();

    println!("{}", a + marker);
}
