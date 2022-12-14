use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
    ops::RangeInclusive,
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

fn solve() {
    let lines = lines_from_file("tin");

    let mut columns: HashMap<u32, Vec<RangeInclusive<u32>>> = HashMap::new();
    let mut rows: HashMap<u32, Vec<RangeInclusive<u32>>> = HashMap::new();
    for line in lines {
        let mut prev_option: Option<(u32, u32)> = None;
        let paths = line.split(" -> ");
        for path in paths {
            let parts: Vec<&str> = path.split(",").collect();
            let p0 = parts[0].parse::<u32>().unwrap();
            let p1 = parts[1].parse::<u32>().unwrap();

            if let Some(prev) = prev_option {
                if prev.0 == p0 {
                    let ranges = columns.entry(p0).or_insert(Vec::new());
                    ranges.push(cmp::min(prev.1, p1)..=cmp::max(prev.1, p1));
                } else {
                    let ranges = rows.entry(p1).or_insert(Vec::new());
                    ranges.push(cmp::min(prev.0, p0)..=cmp::max(prev.0, p0));
                }
            }

            prev_option = Some((p0, p1));
        }
    }

    let bottom_row = *rows.keys().max().unwrap();

    let mut puzzle = Puzzle {
        columns,
        rows,
        bottom_row,
        occupied: HashSet::new(),
        position: Coord(500, 0),
    };

    puzzle.start();
    println!("{:?}", puzzle.occupied.len());
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(u32, u32);

struct Puzzle {
    columns: HashMap<u32, Vec<RangeInclusive<u32>>>,
    rows: HashMap<u32, Vec<RangeInclusive<u32>>>,
    bottom_row: u32,
    occupied: HashSet<Coord>,
    position: Coord,
}

impl Puzzle {
    fn next(&self) -> Option<Coord> {
        let down = Coord(self.position.0, self.position.1 + 1);
        if self.test_candidate(&down) {
            return Some(down);
        }

        let left_down = Coord(self.position.0 - 1, self.position.1 + 1);
        if self.test_candidate(&left_down) {
            return Some(left_down);
        }

        let right_down = Coord(self.position.0 + 1, self.position.1 + 1);
        if self.test_candidate(&right_down) {
            return Some(right_down);
        }

        None
    }

    fn test_candidate(&self, candidate: &Coord) -> bool {
        if self.occupied.contains(&candidate) {
            return false;
        }

        if let Some(row_ranges) = self.columns.get(&candidate.0) {
            for range in row_ranges {
                if range.contains(&candidate.1) {
                    return false;
                }
            }
        }

        if let Some(column_ranges) = self.rows.get(&candidate.1) {
            for range in column_ranges {
                if range.contains(&candidate.0) {
                    return false;
                }
            }
        }

        true
    }

    fn start(&mut self) {
        loop {
            loop {
                if self.position.1 == self.bottom_row {
                    return;
                }
                let next_position = self.next();
                if next_position.is_none() {
                    let _ = &self.occupied.insert(self.position.clone());
                    break;
                }

                self.position = next_position.unwrap();
            }
            self.position = Coord(500, 0);
        }
    }
}
