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
    let lines = lines_from_file("in");

    let mut result = 0;
    for line in lines {
        let p: Vec<&str> = line.split(",").collect();

        let s1: Vec<&str> = p[0].split("-").collect();
        let s2: Vec<&str> = p[1].split("-").collect();

        let a1 = s1[0].parse::<u32>().unwrap();
        let a2 = s1[1].parse::<u32>().unwrap();
        let b1 = s2[0].parse::<u32>().unwrap();
        let b2 = s2[1].parse::<u32>().unwrap();

        let r1 = a1..(a2 + 1);
        let r2 = b1..(b2 + 1);

        if r1.contains(&b1) || r1.contains(&b2) || r2.contains(&a1) || r2.contains(&a2) {
            result += 1;
        }
    }

    println!("{:?}", result);
}
