use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

fn solve() {
    let lines = lines_from_file("in");
    let mut lines_it = lines.iter();

    let mut result: Vec<Vec<&str>> = Vec::new();

    let mut x: i32 = 1;
    let mut read = 0;
    let mut change = 0;
    for cycle in 0..240 {
        if cycle == read {
            x += change;
            let s: Vec<&str> = lines_it.next().unwrap().split(" ").collect();
            if s[0] == "addx" {
                let nr = s[1].parse::<i32>().unwrap();
                change = nr;
                read += 2;
            } else {
                change = 0;
                read += 1;
            }
        }

        let crt_x = cycle % 40;
        if crt_x == 0 {
            result.push(Vec::new());
        }

        let draw = if ((x - 1)..=(x + 1)).contains(&(crt_x as i32)) {
            "██"
        } else {
            "░░"
        };
        result.last_mut().unwrap().push(draw);
    }

    for r in result {
        println!("{}", r.join(""));
    }
}
