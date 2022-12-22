use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(usize, usize);

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Left,
    Right,
}

#[derive(Debug)]
enum Facing {
    North,
    East,
    South,
    West,
}

fn solve() {
    let mut puzzle = parse();
    puzzle.walk();
}

struct Puzzle {
    map: HashMap<Coord, bool>,
    row_ranges: Vec<(usize, usize)>,
    column_ranges: Vec<(usize, usize)>,
    instructions: VecDeque<Instruction>,
    position: Coord,
    facing: Facing,
}

impl Puzzle {
    fn walk(&mut self) {
        while let Some(instruction) = self.instructions.pop_front() {
            match instruction {
                Instruction::Forward(nr) => self.forward(nr),
                Instruction::Left => self.turn_counter_clockwise(),
                Instruction::Right => self.turn_clockwise(),
            }
        }
        println!("{}", self.score());
    }

    fn score(&self) -> usize {
        let facing_score = match self.facing {
            Facing::North => 3,
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
        };

        4 * (self.position.0 + 1) + 1_000 * (self.position.1 + 1) + facing_score
    }

    fn turn_clockwise(&mut self) {
        self.facing = match self.facing {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        };
    }

    fn turn_counter_clockwise(&mut self) {
        self.facing = match self.facing {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        };
    }

    fn forward(&mut self, nr: usize) {
        match self.facing {
            Facing::North => self.forward_until_blocks(nr, false, Puzzle::next_in_column),
            Facing::East => self.forward_until_blocks(nr, true, Puzzle::next_in_row),
            Facing::South => self.forward_until_blocks(nr, true, Puzzle::next_in_column),
            Facing::West => self.forward_until_blocks(nr, false, Puzzle::next_in_row),
        }
    }

    fn next_in_row(&mut self, x: usize, y: usize, east: bool) -> Coord {
        let r = self.row_ranges[y];
        let min = r.0;
        let max = r.1;

        if east {
            if x < max {
                Coord(x + 1, y)
            } else {
                Coord(min, y)
            }
        } else {
            if x > min {
                Coord(x - 1, y)
            } else {
                Coord(max, y)
            }
        }
    }

    fn next_in_column(&mut self, x: usize, y: usize, south: bool) -> Coord {
        let r = self.column_ranges[x];
        let min = r.0;
        let max = r.1;

        if south {
            if y < max {
                Coord(x, y + 1)
            } else {
                Coord(x, min)
            }
        } else {
            if y > min {
                Coord(x, y - 1)
            } else {
                Coord(x, max)
            }
        }
    }

    fn forward_until_blocks<F: Fn(&mut Puzzle, usize, usize, bool) -> Coord>(
        &mut self,
        nr: usize,
        increase: bool,
        f: F,
    ) {
        for _ in 0..nr {
            let candidate = f(self, self.position.0, self.position.1, increase);
            let accessible_option = self.map.get(&candidate);
            if let Some(accessible) = accessible_option {
                if !accessible {
                    return;
                }

                self.position = candidate;
            }
        }
    }
}

fn parse() -> Puzzle {
    let lines = lines_from_file("in");
    let mut puzzle_parts = lines.split(|e| e.is_empty());

    let map_start = parse_map(&puzzle_parts.next().unwrap());
    let map = map_start.0;
    let position = map_start.1;
    let ranges = map_ranges(map.keys().into_iter().collect_vec());

    let instructions = parse_instructions(&puzzle_parts.next().unwrap()[0]);

    Puzzle {
        map,
        row_ranges: ranges.0,
        column_ranges: ranges.1,
        instructions,
        position,
        facing: Facing::East,
    }
}

fn map_ranges(keys: Vec<&Coord>) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let max_x = keys.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = keys.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let mut row_ranges: Vec<(usize, usize)> = Vec::new();
    for y in 0..=max_y {
        let min_row_x = keys
            .iter()
            .filter(|c| c.1 == y)
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .0;

        let max_row_x = keys
            .iter()
            .filter(|c| c.1 == y)
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .0;

        row_ranges.push((min_row_x, max_row_x));
    }

    let mut column_ranges: Vec<(usize, usize)> = Vec::new();
    for x in 0..=max_x {
        let min_row_y = keys
            .iter()
            .filter(|c| c.0 == x)
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;

        let max_row_y = keys
            .iter()
            .filter(|c| c.0 == x)
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;

        column_ranges.push((min_row_y, max_row_y));
    }

    (row_ranges, column_ranges)
}

fn parse_map(lines: &[String]) -> (HashMap<Coord, bool>, Coord) {
    let mut result: HashMap<Coord, bool> = HashMap::new();
    let mut start: Option<Coord> = None;
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let element_option: Option<bool> = match char {
                '.' => Some(true),
                '#' => Some(false),
                _ => None,
            };

            if let Some(element) = element_option {
                if start == None && element {
                    start = Some(Coord(x, y));
                }
                result.insert(Coord(x, y), element);
            }
        }
    }

    (result, start.unwrap())
}

fn parse_instructions(line: &str) -> VecDeque<Instruction> {
    let mut result: VecDeque<Instruction> = VecDeque::new();
    let mut to_process = line;

    while !to_process.is_empty() {
        let re = Regex::new(r"^(\d*).*").unwrap();
        let cap = re.captures(&to_process).unwrap();
        let cap_str = cap.get(1).unwrap().as_str();
        let nr_option = cap_str.parse::<usize>();
        if let Ok(nr) = nr_option {
            result.push_back(Instruction::Forward(nr));
            to_process = &to_process[cap_str.len()..];
        } else {
            result.push_back(match &to_process[..1] {
                "L" => Instruction::Left,
                "R" => Instruction::Right,
                _ => panic!("Parsing error"),
            });
            to_process = &to_process[1..];
        }
    }

    result
}
