use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|e| e.expect("could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_file("in");

    let mut puzzle_parts = lines.split(|e| e.is_empty());
    let stack_lines = puzzle_parts.next().unwrap();

    let nr_of_stacks = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<&str>> = (0..nr_of_stacks).map(|_| Vec::new()).collect();
    for line in stack_lines.split_last().unwrap().1 {
        for i in 0..nr_of_stacks {
            let position = 1 + 4 * i;
            let item = &line[position..(position + 1)].trim();
            if !item.is_empty() {
                stacks[i].push(item);
            }
        }
    }

    let move_lines = puzzle_parts.next().unwrap();
    for line in move_lines {
        let elements: Vec<&str> = line.split(" ").collect();
        let moves = elements[1].parse::<u32>().unwrap();
        let from = elements[3].parse::<u32>().unwrap() - 1;
        let into = elements[5].parse::<u32>().unwrap() - 1;

        for n in 0..moves {
            let x = stacks[from as usize].remove((moves - n - 1) as usize);
            stacks[into as usize].insert(0, x);
        }
    }

    println!("{}", stacks.iter().map(|s| s[0]).join(""));
}
