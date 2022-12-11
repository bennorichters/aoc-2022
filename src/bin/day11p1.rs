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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    test_pass: usize,
    test_fail: usize,
}

fn solve() {
    let mut monkeys = parse();

    let mut activity: Vec<u32> = (0..monkeys.len()).map(|_| 0).collect();
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let mut transfers: Vec<(u32, usize)> = Vec::new();
            let monkey = &monkeys[m];

            activity[m] += *(&monkey.items.len()) as u32;
            for item in &monkey.items {
                let worry_level = match monkey.operation {
                    Operation::Add(value) => item + value,
                    Operation::Multiply(value) => item * value,
                    Operation::Square => item * item,
                } / 3;

                let to = if worry_level % monkey.test == 0 {
                    monkey.test_pass
                } else {
                    monkey.test_fail
                };

                transfers.push((worry_level, to));
            }

            monkeys[m].items.clear();
            for transfer in &transfers {
                monkeys[transfer.1].items.push(transfer.0);
            }

        }
    }

    for m in monkeys {
        println!("{:?}", m);
    }
    activity.sort();
    activity.reverse();
    println!("{}", activity[0] * activity[1]);
}

fn parse() -> Vec<Monkey> {
    let lines = lines_from_file("in");
    let iter = lines.split(|e| e.is_empty());

    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_lines in iter {
        let items: Vec<u32> = (&monkey_lines[1]["  Starting items: ".len()..])
            .split(",")
            .map(|i| i.trim().parse::<u32>().unwrap())
            .collect();

        let operation_line = &monkey_lines[2]["  Operation: new = old ".len()..];
        let op_split: Vec<&str> = operation_line.split_ascii_whitespace().collect();
        let operation = parse_operation(&op_split[0], &op_split[1]);

        let test = monkey_lines[3]
            .split(" ")
            .last()
            .unwrap()
            .parse::<u32>()
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
            items: items.clone(),
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

    let v = p1.parse::<u32>().unwrap();
    if p0 == "+" {
        return Operation::Add(v);
    }

    Operation::Multiply(v)
}
