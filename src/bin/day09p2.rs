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

    let mut ks: Vec<Coord> = (0..10).map(|_| Coord(0, 0)).collect();
    let mut visited: HashSet<Coord> = HashSet::new();

    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();

        let m = s[0];
        let n = s[1].parse::<u32>().unwrap();
        for _ in 0..n {
            match m {
                "U" => ks[0] = Coord(ks[0].0, ks[0].1 - 1),
                "R" => ks[0] = Coord(ks[0].0 + 1, ks[0].1),
                "D" => ks[0] = Coord(ks[0].0, ks[0].1 + 1),
                "L" => ks[0] = Coord(ks[0].0 - 1, ks[0].1),
                _ => panic!(""),
            }

            for k in 1..10 {
                if (ks[k - 1].0 - ks[k].0).abs() == 2 && (ks[k - 1].1 - ks[k].1).abs() == 2 {
                    ks[k] = Coord(
                        ks[k].0 + (ks[k - 1].0 - ks[k].0) / 2,
                        ks[k].1 + (ks[k - 1].1 - ks[k].1) / 2,
                    );
                } else {
                    if (ks[k - 1].0 - ks[k].0).abs() == 2 {
                        ks[k] = Coord(ks[k].0 + (ks[k - 1].0 - ks[k].0) / 2, ks[k].1);
                        if ks[k - 1].1 != ks[k].1 {
                            ks[k] = Coord(ks[k].0, ks[k - 1].1);
                        }
                    }

                    if (ks[k - 1].1 - ks[k].1).abs() == 2 {
                        ks[k] = Coord(ks[k].0, ks[k].1 + (ks[k - 1].1 - ks[k].1) / 2);
                        if ks[k - 1].0 != ks[k].0 {
                            ks[k] = Coord(ks[k - 1].0, ks[k].1);
                        }
                    }
                }
            }

            visited.insert(ks[9].clone());
        }
    }

    println!("{}", visited.len());
}
