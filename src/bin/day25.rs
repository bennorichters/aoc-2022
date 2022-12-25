use std::{
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
    let i = nr();

    let s = snafu(i);
    let b = snafu_to_int(s.to_owned());
    println!("{}, {}, {}", i, b, s);
}

fn snafu(nr: u64) -> String {
    let mut result: Vec<char> = Vec::new();
    let tokens = vec!['0', '1', '2', '=', '-'];

    let mut rest = nr;
    loop {
        let rem = rest % 5;
        result.push(tokens[rem as usize]);

        if rem > 2 {
            rest += 5;
        }
        rest /= 5;
        if rest == 0 {
            break;
        }
    }

    result.iter().rev().collect()
}

fn nr() -> u64 {
    let lines = lines_from_file("in");

    let mut total: i64 = 0;
    for line in lines {
        let nr = snafu_to_int(line);
        total += nr;
    }

    total as u64
}

fn snafu_to_int(line: String) -> i64 {
    let mut nr: i64 = 0;
    for (i, c) in line.chars().rev().enumerate() {
        let g: i64 = (5i64).pow(i as u32);
        let n: i64 = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        };

        nr += g * n;
    }

    nr
}
