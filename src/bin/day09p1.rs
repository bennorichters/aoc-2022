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
        .map(|e| e.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(i32, i32);

fn solve() {
    let lines = lines_from_file("in");

    let mut h = Coord(0, 0);
    let mut t = Coord(0, 0);
    let mut visited: HashSet<Coord> = HashSet::new();

    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();

        let m = s[0];
        let n = s[1].parse::<u32>().unwrap();
        for _ in 0..n {
            h = match m {
                "U" => Coord(h.0, h.1 - 1),
                "R" => Coord(h.0 + 1, h.1),
                "D" => Coord(h.0, h.1 + 1),
                "L" => Coord(h.0 - 1, h.1),
                _ => panic!(""),
            };

            if (h.0 - t.0).abs() == 2 || (h.1 - t.1).abs() == 2 {
                let dx = (h.0 - t.0).signum();
                let dy = (h.1 - t.1).signum();
                t = Coord(t.0 + dx, t.1 + dy);

            }
            visited.insert(t.clone());
        }
    }

    println!("{}", visited.len());
}
