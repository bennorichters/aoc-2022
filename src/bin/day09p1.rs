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
            match m {
                "U" => h = Coord(h.0, h.1 - 1),
                "R" => h = Coord(h.0 + 1, h.1),
                "D" => h = Coord(h.0, h.1 + 1),
                "L" => h = Coord(h.0 - 1, h.1),
                _ => panic!(""),
            }

            if (h.0 - t.0).abs() == 2 {
                t = Coord(t.0 + (h.0 - t.0) / 2, t.1);
                if h.1 != t.1 {
                    t = Coord(t.0, h.1);
                }
            }

            if (h.1 - t.1).abs() == 2 {
                t = Coord(t.0, t.1 + (h.1 - t.1) / 2);
                if h.0 != t.0 {
                    t = Coord(h.0, t.1);
                }
            }

            visited.insert(t.clone());
        }
    }

    println!("{}", visited.len());
}
