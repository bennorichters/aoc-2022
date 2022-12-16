use std::{
    cmp,
    collections::HashMap,
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(i64, i64);

fn solve() {
    let pairs = parse();

    let distances: Vec<(&Coord, i64)> = pairs
        .iter()
        .map(|p| (&p.0, calc_distance(&p.0, &p.1)))
        .collect();

    let mut ranges_per_row: HashMap<i64, Vec<RangeInclusive<i64>>> = HashMap::new();
    for dis in distances {
        let s = dis.0;
        let d = dis.1;

        let mut length = 1;
        for row in (s.1 - d)..=(s.1 + d) {
            let ranges = ranges_per_row.entry(row).or_insert(Vec::new());
            let range = (s.0 - length / 2)..=(s.0 + length / 2);
            let merged = merge_add_ranges(ranges, range);
            ranges_per_row.insert(row, merged);
            length += if row < s.1 { 2 } else { -2 };
        }
    }

    let foo = 4_000_000;
    for row in 0..=foo {
        let ranges_option = ranges_per_row.get(&row);
        if let Some(ranges) = ranges_option {
            if ranges.len() > 1 {
                println!("{}, {:?}", row, ranges);
                let x = ranges[0].end() + 1;
                let result = foo * x + row;
                println!("{}", result);
            }
        }
    }
}

fn merge_add_ranges(
    ranges: &mut Vec<RangeInclusive<i64>>,
    extra: RangeInclusive<i64>,
) -> Vec<RangeInclusive<i64>> {
    let mut result: Vec<RangeInclusive<i64>> = Vec::new();

    ranges.reverse();
    let mut to_merge = extra;
    while !ranges.is_empty() {
        let first = ranges.pop().unwrap();

        if ranges_overlap(&first, &to_merge) {
            to_merge = merge_ranges(&first, &to_merge);
        } else if to_merge.start() < first.start() {
            result.push(to_merge);
            result.push(first);
            result.append(ranges);
            return result;
        } else {
            result.push(first);
        }
    }

    result.push(to_merge);
    result
}

fn ranges_overlap(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    r1.contains(&(r2.start() - 1))
        || r1.contains(&(r2.end() + 1))
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

fn merge_ranges(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    cmp::min(*r1.start(), *r2.start())..=cmp::max(*r1.end(), *r2.end())
}

fn calc_distance(c1: &Coord, c2: &Coord) -> i64 {
    let dist_x = i64::abs(c1.0 - c2.0);
    let dist_y = i64::abs(c1.1 - c2.1);

    dist_x + dist_y
}

fn parse() -> Vec<(Coord, Coord)> {
    let lines = lines_from_file("in");

    let mut result: Vec<(Coord, Coord)> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let sensor = parse_coord(parts[0]);
        let beacon = parse_coord(parts[1]);
        result.push((sensor, beacon));
    }

    result
}

fn parse_coord(txt: &str) -> Coord {
    let parts: Vec<&str> = txt.split(", ").collect();
    let x = parse_number(parts[0]);
    let y = parse_number(parts[1]);

    Coord(x, y)
}

fn parse_number(txt: &str) -> i64 {
    *(&txt[(&txt.find("=").unwrap() + 1)..].parse::<i64>().unwrap())
}
