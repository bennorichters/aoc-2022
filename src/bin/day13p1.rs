use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use json::JsonValue;

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
    let lines = lines_from_file("in");
    let puzzle_parts = lines.split(|e| e.is_empty());

    let mut result = 0;
    for (i, batch) in puzzle_parts.enumerate() {
        let j1 = json::parse(&batch[0]).unwrap();
        let j2 = json::parse(&batch[1]).unwrap();

        if check(&j1, &j2).unwrap() {
            result += i + 1;
        }
    }

    println!("{:?}", result);
}

fn check(j1: &JsonValue, j2: &JsonValue) -> Option<bool> {
    for (i, m1) in j1.members().enumerate() {
        let m2op = j2.members().nth(i);
        if m2op.is_none() {
            return Some(false);
        }
        let m2 = m2op.unwrap();

        if m1.is_number() && m2.is_number() {
            let n1 = m1.as_u8().unwrap();
            let n2 = m2.as_u8().unwrap();

            if n1 < n2 {
                return Some(true);
            } else if n1 > n2 {
                return Some(false);
            }
        } else if m1.is_number() {
            let mut sub_arr = JsonValue::new_array();
            sub_arr.push(m1.clone()).unwrap();
            let sub = check(&sub_arr, m2);
            if sub.is_some() {
                return sub;
            }
        } else if m2.is_number() {
            let mut sub_arr = JsonValue::new_array();
            sub_arr.push(m2.clone()).unwrap();
            let sub = check(m1, &sub_arr);
            if sub.is_some() {
                return sub;
            }
        } else {
            let sub = check(m1, m2);
            if sub.is_some() {
                return sub;
            }
        }
    }

    if j1.members().len() == j2.members().len() {
        None
    } else {
        Some(true)
    }
}
