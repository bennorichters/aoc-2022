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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: u64,
    test_pass: usize,
    test_fail: usize,
}

#[derive(Debug)]
struct Item {
    value: u64,
    executed_operations: Vec<Operation>,
}

fn solve() {
    let mut monkeys = parse();
    throw_items(&mut monkeys);
}

fn throw_items(monkeys: &mut Vec<Monkey>) {
    let mut activity = vec![0u64; monkeys.len()];
    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            activity[m] += *(&monkey.items.len()) as u64;

            let mut transfers: Vec<(Item, usize)> = Vec::new();
            while !monkey.items.is_empty() {
                let mut item = monkey.items.pop().unwrap();
                item.executed_operations.push(monkey.operation.clone());
                let to = if test_item(monkey.test, &item) {
                    monkey.test_pass
                } else {
                    monkey.test_fail
                };

                transfers.push((item, to));
            }

            monkeys[m].items.clear();
            while !transfers.is_empty() {
                let transfer = transfers.pop().unwrap();
                monkeys[transfer.1].items.push(transfer.0);
            }
        }
    }

    activity.sort();
    activity.reverse();
    println!("{}", activity[0] * activity[1]);
}

fn test_item(test: u64, item: &Item) -> bool {
    let mut remainder = item.value % test;

    for op in &item.executed_operations {
        remainder = match op {
            Operation::Add(value) => (remainder % test) + value,
            Operation::Multiply(value) => (remainder % test) * value,
            Operation::Square => (remainder % test) * remainder,
        };
    }

    remainder % test == 0
}

fn parse() -> Vec<Monkey> {
    let lines = lines_from_file("tin");
    let iter = lines.split(|e| e.is_empty());

    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_lines in iter {
        let items: Vec<Item> = (&monkey_lines[1]["  Starting items: ".len()..])
            .split(",")
            .map(|i| Item {
                value: i.trim().parse::<u64>().unwrap(),
                executed_operations: vec![],
            })
            .collect();

        let operation_line = &monkey_lines[2]["  Operation: new = old ".len()..];
        let op_split: Vec<&str> = operation_line.split_ascii_whitespace().collect();
        let operation = parse_operation(&op_split[0], &op_split[1]);

        let test = monkey_lines[3]
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let test_pass = monkey_lines[4]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let test_fail = monkey_lines[5]
            .split(" ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            test_pass,
            test_fail,
        });
    }

    monkeys
}

fn parse_operation(p0: &str, p1: &str) -> Operation {
    if p1 == "old" {
        return Operation::Square;
    }

    let v = p1.parse::<u64>().unwrap();
    if p0 == "+" {
        return Operation::Add(v);
    }

    Operation::Multiply(v)
}
