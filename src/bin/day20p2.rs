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

const KEY: i64 = 811589153;

fn solve() {
    let lines = lines_from_file("in");
    let nrs: Vec<i64> = lines
        .iter()
        .map(|n| KEY * n.parse::<i64>().unwrap())
        .collect();
    let length = nrs.len() as i64;

    let mut indices: Vec<usize> = (0..length as usize).collect();
    let mut decoded = nrs.clone();

    for _round in 0..10 {
        for k in 0..indices.len() {
            let index = indices[k];
            let nr = decoded[index];

            let shifted = (index as i64) + nr;
            let mut wrapped = shifted % (length - 1);
            if wrapped != (index as i64) {
                if wrapped < 0 {
                    wrapped = length + wrapped - 1;
                }
                decoded.remove(index);
                decoded.insert(wrapped as usize, nr);

                for j in 0..indices.len() {
                    if wrapped as usize > index {
                        if ((index + 1)..=(wrapped as usize)).contains(&indices[j]) {
                            indices[j] -= 1;
                        }
                    } else if ((wrapped as usize)..index).contains(&indices[j]) {
                        indices[j] += 1;
                    }
                }
                indices[k] = wrapped as usize;
            }
        }
    }

    let zero = decoded.iter().position(|&e| e == 0).unwrap();
    let r1 = decoded.iter().cycle().nth(zero + 1000).unwrap();
    let r2 = decoded.iter().cycle().nth(zero + 2000).unwrap();
    let r3 = decoded.iter().cycle().nth(zero + 3000).unwrap();
    println!("{}, {}, {}", r1, r2, r3);
    println!("{}", r1 + r2 + r3);
}
