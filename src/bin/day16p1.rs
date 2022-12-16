#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;

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
    let parsed = parse();
    for p in parsed {
        println!("{:?}", p);
    }
}

fn parse() -> Vec<(u64, Vec<usize>)> {
    let lines = lines_from_file("tin");

    let mut raw: Vec<(String, u64, Vec<String>)> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let valve = parts[1].to_owned();
        let connects: Vec<String> = parts[9..]
            .iter()
            .cloned()
            .map(|p| p.replace(",", "").to_owned())
            .collect();

        let re = Regex::new(r"\d+").unwrap();
        let cap = re.captures(&line).unwrap();
        let flow_rate = cap.get(0).unwrap().as_str().parse::<u64>().unwrap();

        raw.push((valve, flow_rate, connects));
    }

    raw.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result: Vec<(u64, Vec<usize>)> = Vec::new();
    for raw_element in &raw {
        let mut connecting_indices: Vec<usize> = Vec::new();
        for connecting_valve in &raw_element.2 {
            let i = raw.iter().position(|e| &e.0 == connecting_valve).unwrap();
            connecting_indices.push(i);
        }

        connecting_indices.sort();
        result.push((raw_element.1, connecting_indices));
    }

    result
}
