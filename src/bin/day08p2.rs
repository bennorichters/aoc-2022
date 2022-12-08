use std::{
    cmp::{self, *},
    collections::HashMap,
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
            map.insert(Coord(x as u8, y as u8), height);
        }
    }

    let mut result = 0;
    for x in 0..size {
        for y in 0..size {
            let mut scenic = 1;
            let height = map.get(&Coord(x as u8, y as u8)).unwrap();

            let mut vis = 0;
            for xx in (0..x).rev() {
                vis += 1;
                let test = map.get(&Coord(xx as u8, y as u8)).unwrap();
                if test >= height {
                    break;
                }
            }
            scenic *= vis;

            let mut vis = 0;
            for xx in (x + 1)..size {
                vis += 1;
                let test = map.get(&Coord(xx as u8, y as u8)).unwrap();
                if test >= height {
                    break;
                }
            }
            scenic *= vis;

            let mut vis = 0;
            for yy in (0..y).rev() {
                vis += 1;
                let test = map.get(&Coord(x as u8, yy as u8)).unwrap();
                if test >= height {
                    break;
                }
            }
            scenic *= vis;

            let mut vis = 0;
            for yy in (y + 1)..size {
                vis += 1;
                let test = map.get(&Coord(x as u8, yy as u8)).unwrap();
                if test >= height {
                    break;
                }
            }
            scenic *= vis;

            result = cmp::max(result, scenic);
        }
    }

    println!("{:?}", result);
}

