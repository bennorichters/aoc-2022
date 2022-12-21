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

    let mut resolved: HashMap<String, i64> = HashMap::new();
    let mut state_stack: Vec<String> = monkeys.keys().map(|m| m.to_owned()).collect();
    while !state_stack.is_empty() {
        let monkey = state_stack.pop().unwrap();
        let operation = monkeys.get(&monkey.to_owned()).unwrap();
        if let Some(result) = calc(&operation, &resolved) {
            if monkey == "root" {
                println!("{}", result);
                return;
            }

            resolved.insert(monkey.to_owned(), result);
        } else {
            state_stack.insert(0, monkey);
        }
    }
}

fn calc(op: &Operation, resolved: &HashMap<String, i64>) -> Option<i64> {
    match op {
        Operation::Add(m1, m2) => {
            if let (Some(sub_op1), Some(sub_op2)) =
                (resolved.get(&m1.to_owned()), resolved.get(&m2.to_owned()))
            {
                Some(sub_op1 + sub_op2)
            } else {
                None
            }
        }
        Operation::Divide(m1, m2) => {
            if let (Some(sub_op1), Some(sub_op2)) =
                (resolved.get(&m1.to_owned()), resolved.get(&m2.to_owned()))
            {
                Some(sub_op1 / sub_op2)
            } else {
                None
            }
        }
        Operation::Multiply(m1, m2) => {
            if let (Some(sub_op1), Some(sub_op2)) =
                (resolved.get(&m1.to_owned()), resolved.get(&m2.to_owned()))
            {
                Some(sub_op1 * sub_op2)
            } else {
                None
            }
        }
        Operation::Number(nr) => Some(*nr),
        Operation::Subtract(m1, m2) => {
            if let (Some(sub_op1), Some(sub_op2)) =
                (resolved.get(&m1.to_owned()), resolved.get(&m2.to_owned()))
            {
                Some(sub_op1 - sub_op2)
            } else {
                None
            }
        }
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
