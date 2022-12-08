use std::{
    cmp::{self, *},
    collections::{HashMap, HashSet},
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

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord(u8, u8);

fn solve() {
    let lines = lines_from_file("in");

    let mut map: HashMap<Coord, u8> = HashMap::new();

    let size = lines[0].len();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let height = char as u8 - '0' as u8;
            map.insert(Coord(x as u8, y as u8), height + 1);
        }
    }

    let mut all: HashSet<Coord> = HashSet::new();

    for x in 0..size {
        let mut test: u8 = 0;
        for y in 0..size {
            let c = Coord(x as u8, (size - 1 - y) as u8);
            let height = map.get(&c).unwrap();
            if test < *height {
                all.insert(c);
            }
            test = cmp::max(test, *height);
        }
    }

    for y in 0..size {
        let mut test: u8 = 0;
        for x in 0..size {
            let c = Coord((size - x - 1) as u8, y as u8);
            let height = map.get(&c).unwrap();
            if test < *height {
                all.insert(c);
            }
            test = cmp::max(test, *height);
        }
    }

    for x in 0..size {
        let mut test: u8 = 0;
        for y in 0..size {
            let c = Coord(x as u8, y as u8);
            let height = map.get(&c).unwrap();
            if test < *height {
                all.insert(c);
            }
            test = cmp::max(test, *height);
        }
    }

    for y in 0..size {
        let mut test: u8 = 0;
        for x in 0..size {
            let c = Coord(x as u8, y as u8);
            let height = map.get(&c).unwrap();
            if test < *height {
                all.insert(c);
            }
            test = cmp::max(test, *height);
        }
    }

    println!("{:?}", all.len());
}
