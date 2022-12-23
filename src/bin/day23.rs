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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(i64, i64);

impl Coord {
    fn neighbour(&self, direction: Direction) -> Coord {
        match direction {
            Direction::N => Coord(self.0, self.1 - 1),
            Direction::NE => Coord(self.0 + 1, self.1 - 1),
            Direction::E => Coord(self.0 + 1, self.1),
            Direction::SE => Coord(self.0 + 1, self.1 + 1),
            Direction::S => Coord(self.0, self.1 + 1),
            Direction::SW => Coord(self.0 - 1, self.1 + 1),
            Direction::W => Coord(self.0 - 1, self.1),
            Direction::NW => Coord(self.0 - 1, self.1 - 1),
        }
    }

    fn three_neighbours(&self, directions: Peek) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();

        result.push(self.neighbour(directions.0));
        result.push(self.neighbour(directions.1));
        result.push(self.neighbour(directions.2));

        result
    }

    fn all_neigbours(&self) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();

        result.push(self.neighbour(Direction::N));
        result.push(self.neighbour(Direction::NE));
        result.push(self.neighbour(Direction::E));
        result.push(self.neighbour(Direction::SE));
        result.push(self.neighbour(Direction::S));
        result.push(self.neighbour(Direction::SW));
        result.push(self.neighbour(Direction::W));
        result.push(self.neighbour(Direction::NW));

        result
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

type Peek = (Direction, Direction, Direction);

const PEEKS: [&'static Peek; 4] = [
    &(Direction::NW, Direction::N, Direction::NE),
    &(Direction::SW, Direction::S, Direction::SE),
    &(Direction::SW, Direction::W, Direction::NW),
    &(Direction::SE, Direction::E, Direction::NE),
];

fn next_peeks_index(old: usize) -> usize {
    if old < PEEKS.len() - 1 {
        old + 1
    } else {
        0
    }
}

fn solve() {
    let elves = parse();
    let mut puzzle = Puzzle {
        peek_index: 0,
        elves,
        someone_moved: true,
    };

    let mut round = 0;
    while puzzle.someone_moved {
        round += 1;
        puzzle.step();

        println!("{}, {}", round, puzzle.empty_count());
    }

    println!("{}", round);
}

struct Puzzle {
    peek_index: usize,
    elves: Vec<Coord>,
    someone_moved: bool,
}

impl Puzzle {
    fn step(&mut self) {
        self.someone_moved = false;

        let ps = self.proposals();
        let mut fresh: Vec<Coord> = Vec::new();
        for elve in &self.elves {
            if let Some(update) = ps.get(&elve) {
                self.someone_moved = true;
                fresh.push(update.clone());
            } else {
                fresh.push(elve.clone());
            }
        }

        self.elves = fresh;
        self.peek_index = next_peeks_index(self.peek_index);
    }

    fn proposals(&self) -> HashMap<Coord, Coord> {
        let mut proposals: HashMap<Coord, Coord> = HashMap::new();
        for elve in &self.elves {
            let all_neigbours = elve.all_neigbours();
            if !self.all_empty(&all_neigbours) {
                let proposal_option = self.proposal_for_elve(&elve);
                if let Some(proposal) = proposal_option {
                    proposals.insert(elve.clone(), proposal);
                }
            }
        }

        let vs: Vec<Coord> = proposals.values().map(|v| v.clone()).collect();
        proposals.retain(|_, v| vs.iter().filter(|c| c == &v).count() == 1);
        proposals
    }

    fn proposal_for_elve(&self, elve: &Coord) -> Option<Coord> {
        let mut pi = self.peek_index;
        for _ in 0..=PEEKS.len() {
            let peek: Peek = *PEEKS[pi];
            let peek_neighbours = elve.three_neighbours(peek);
            if self.all_empty(&peek_neighbours) {
                return Some(peek_neighbours[1].clone());
            }

            pi = next_peeks_index(pi);
        }

        None
    }

    fn all_empty(&self, coords: &Vec<Coord>) -> bool {
        coords.iter().filter(|c| self.elves.contains(c)).count() == 0
    }

    fn top_left_right_bottom(&self) -> (Coord, Coord) {
        let min_x = self.elves.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let max_x = self.elves.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let min_y = self.elves.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
        let max_y = self.elves.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

        (Coord(min_x, min_y), Coord(max_x, max_y))
    }

    fn empty_count(&self) -> usize {
        let edges = self.top_left_right_bottom();
        let width = edges.1 .0 - edges.0 .0 + 1;
        let height = edges.1 .1 - edges.0 .1 + 1;
        let area = width * height;

        area as usize - self.elves.len()
    }

    fn print_elves(&self) {
        let edges = self.top_left_right_bottom();
        for y in edges.0 .1..=edges.1 .1 {
            for x in edges.0 .0..=edges.1 .0 {
                print!(
                    "{}",
                    if self.elves.contains(&Coord(x, y)) {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!();
        }
    }
}

fn parse() -> Vec<Coord> {
    let lines = lines_from_file("in");

    let mut elves: Vec<Coord> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.push(Coord(x as i64, y as i64));
            }
        }
    }

    elves
}
