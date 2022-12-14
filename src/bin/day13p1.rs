#![allow(dead_code)]
#![allow(unused_variables)]

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
    // solve();

    let a = json::parse("[[],7]").unwrap();
    let b = json::parse("[[3]]").unwrap();

    let result = check(&a, &b);
    println!("{}", result);
}

// fn solve() {
//     let lines = lines_from_file("tin");
//     let puzzle_parts = lines.split(|e| e.is_empty());

//     let mut result = 0;
//     for (i, batch) in puzzle_parts.enumerate() {
//         let j1 = json::parse(&batch[0]).unwrap();
//         let j2 = json::parse(&batch[1]).unwrap();

//         if check(&j1, &j2) {
//             result += i + 1;
//         }
//     }

//     println!("{:?}", result);
// }

fn check(j1: &JsonValue, j2: &JsonValue) -> bool {
    for (i, m1) in j1.members().enumerate() {
        let m2op = j2.members().nth(i);
        if m2op.is_none() {
            return false;
        }
        let m2 = m2op.unwrap();

        if m1.is_number() && m2.is_number() {
            let n1 = m1.as_u8().unwrap();
            let n2 = m2.as_u8().unwrap();

            if n1 < n2 {
                return true;
            } else if n1 > n2 {
                return false;
            }
        } else if m1.is_number() {
            let mut sub_arr = JsonValue::new_array();
            sub_arr.push(m1.clone()).unwrap();
            if !check(&sub_arr, m2) {
                return false;
            }
        } else if m2.is_number() {
            let mut sub_arr = JsonValue::new_array();
            sub_arr.push(m2.clone()).unwrap();
            if !check(m1, &sub_arr) {
                return false;
            }
        } else if !check(m1, m2) {
            return false;
        }
    }

    true
}
