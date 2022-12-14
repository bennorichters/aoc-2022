use std::{
    cmp::Ordering,
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

    let div1 = "[[2]]";
    let div2 = "[[6]]";

    let mut all: Vec<JsonValue> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            all.push(json::parse(&line).unwrap());
        }
    }
    all.push(json::parse(div1).unwrap());
    all.push(json::parse(div2).unwrap());

    all.sort_by(|a, b| check(a, b).unwrap());

    let mut result = 1;
    for (i, v) in all.iter().enumerate() {
        if v.to_string() == div1 || v.to_string() == div2 {
            result *= i + 1
        }
    }
    println!("{}", result);
}

fn check(j1: &JsonValue, j2: &JsonValue) -> Option<Ordering> {
    for (i, m1) in j1.members().enumerate() {
        let m2op = j2.members().nth(i);
        if m2op.is_none() {
            return Some(Ordering::Greater);
        }
        let m2 = m2op.unwrap();

        if m1.is_number() && m2.is_number() {
            let n1 = m1.as_u8().unwrap();
            let n2 = m2.as_u8().unwrap();

            if n1 < n2 {
                return Some(Ordering::Less);
            } else if n1 > n2 {
                return Some(Ordering::Greater);
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
        Some(Ordering::Less)
    }
}
