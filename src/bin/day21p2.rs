#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashMap,
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

fn solve() {
    let monkeys = parse();

    // let root_eq1 = String::from("pppw");
    // let root_eq2 = String::from("sjmn");
    let root_eq1 = String::from("hsdb");
    let root_eq2 = String::from("mwrd");

    let mut min = 1_000_000_000_000;
    let mut max = 9_000_000_000_000;

    for _ in 0..100 {
        println!("{}, {}", min, max);
        let r = search(min, max, root_eq1.to_owned(), root_eq2.to_owned(), &monkeys);
        min = r.0;
        max = r.1;
    }
}

fn search(
    min: i64,
    max: i64,
    root_eq1: String,
    root_eq2: String,
    monkeys: &HashMap<String, Operation>,
) -> (i64, i64) {
    let candidate = min + (max - min) / 2;
    let mut found1: Option<i64> = None;
    let mut found2: Option<i64> = None;

    let mut resolved: HashMap<String, i64> = HashMap::new();
    resolved.insert(String::from("humn"), candidate);
    let mut state_stack: Vec<String> = monkeys.keys().map(|m| m.to_owned()).collect();
    while !state_stack.is_empty() {
        let monkey = state_stack.pop().unwrap();
        let operation = monkeys.get(&monkey.to_owned()).unwrap();
        if let Some(result) = operate(&operation, &resolved) {
            if monkey == root_eq1 {
                println!("--> {}, {}", monkey, result);
                found1 = Some(result);
            }
            if monkey == root_eq2 {
                println!("--> {}, {}", monkey, result);
                found2 = Some(result);
            }

            if let (Some(r1), Some(r2)) = (found1, found2) {
                if r1 == r2 {
                    println!("SUCCES! {}", candidate);
                }
                if r1 < r2 {
                    return (min, candidate);
                }
                return (candidate, max);
            }

            resolved.insert(monkey.to_owned(), result);
        } else {
            state_stack.insert(0, monkey);
        }
    }

    panic!();
}

fn operate(op: &Operation, resolved: &HashMap<String, i64>) -> Option<i64> {
    match op {
        Operation::Add(m1, m2) => calc(m1, m2, resolved, |v1, v2| v1 + v2),
        Operation::Divide(m1, m2) => calc(m1, m2, resolved, |v1, v2| {
            v1 / v2 + if v1 % v2 == 0 { 0 } else { 1 }
        }),
        Operation::Multiply(m1, m2) => calc(m1, m2, resolved, |v1, v2| v1 * v2),
        Operation::Number(nr) => Some(*nr),
        Operation::Subtract(m1, m2) => calc(m1, m2, resolved, |v1, v2| v1 - v2),
    }
}

fn calc<F: Fn(i64, i64) -> i64>(
    m1: &str,
    m2: &str,
    resolved: &HashMap<String, i64>,
    f: F,
) -> Option<i64> {
    if let (Some(sub_op1), Some(sub_op2)) =
        (resolved.get(&m1.to_owned()), resolved.get(&m2.to_owned()))
    {
        Some(f(*sub_op1, *sub_op2))
    } else {
        None
    }
}

fn parse() -> HashMap<String, Operation> {
    let lines = lines_from_file("in");

    let mut monkeys: HashMap<String, Operation> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();

        let nr_option = parts[1].parse::<i64>();
        if let Ok(nr) = nr_option {
            monkeys.insert(parts[0].to_owned(), Operation::Number(nr));
        } else {
            let op_char = &parts[1][5..6];
            let m1 = parts[1][..4].to_owned();
            let m2 = parts[1][7..].to_owned();
            let op = match op_char {
                "+" => Operation::Add(m1, m2),
                "/" => Operation::Divide(m1, m2),
                "*" => Operation::Multiply(m1, m2),
                "-" => Operation::Subtract(m1, m2),
                _ => panic!("parsing error"),
            };
            monkeys.insert(parts[0].to_owned(), op);
        }
    }

    monkeys
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Operation {
    Add(String, String),
    Divide(String, String),
    Multiply(String, String),
    Number(i64),
    Subtract(String, String),
}
