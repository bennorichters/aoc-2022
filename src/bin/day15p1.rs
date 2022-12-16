use std::{
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
    let pairs = parse();

    let distances: Vec<(&Coord, i32)> = pairs
        .iter()
        .map(|p| (&p.0, calc_distance(&p.0, &p.1)))
        .collect();

    let sensors: Vec<&Coord> = pairs.iter().map(|p| &p.0).collect();
    let beacons: Vec<&Coord> = pairs.iter().map(|p| &p.1).collect();
    let min_sensor_x = distances.iter().map(|s| s.0 .0).min().unwrap();
    let max_sensor_x = distances.iter().map(|s| s.0 .0).max().unwrap();
    let max_dist = distances.iter().map(|d| d.1).max().unwrap();

    // let search_y = 10;
    let search_y = 2_000_000;
    let mut result = 0;
    for x in (min_sensor_x - max_dist)..(max_sensor_x + max_dist) {
        let test_coord = Coord(x, search_y);
        if !sensors.contains(&&test_coord)
            && !beacons.contains(&&test_coord)
            && in_range(&distances, &test_coord)
        {
            result += 1;
        }
    }

    println!("{}", result);
}

fn in_range(distances: &Vec<(&Coord, i32)>, coord: &Coord) -> bool {
    for dist in distances {
        let test_distance = calc_distance(&dist.0, &coord);
        if test_distance <= dist.1 {
            return true;
        }
    }

    false
}

fn calc_distance(c1: &Coord, c2: &Coord) -> i32 {
    let dist_x = i32::abs(c1.0 - c2.0);
    let dist_y = i32::abs(c1.1 - c2.1);

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

fn parse_number(txt: &str) -> i32 {
    *(&txt[(&txt.find("=").unwrap() + 1)..].parse::<i32>().unwrap())
}
