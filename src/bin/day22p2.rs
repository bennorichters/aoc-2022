use std::{
    collections::{HashMap, VecDeque},
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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(usize, usize);

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Facing {
    North,
    East,
    South,
    West,
}

const SQUARE_SIZE: usize = 50;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Region {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn solve() {
    let puzzle = parse();
    puzzle.walk();
}

fn region_changes() -> HashMap<(Region, Facing), (Region, Facing)> {
    HashMap::from([
        ((Region::A, Facing::West), (Region::B, Facing::West)),
        ((Region::A, Facing::North), (Region::F, Facing::North)),
        ((Region::A, Facing::East), (Region::D, Facing::West)),
        ((Region::A, Facing::South), (Region::C, Facing::West)),
        ((Region::B, Facing::East), (Region::A, Facing::East)),
        ((Region::B, Facing::South), (Region::C, Facing::South)),
        ((Region::B, Facing::West), (Region::E, Facing::East)),
        ((Region::B, Facing::North), (Region::F, Facing::East)),
        ((Region::C, Facing::North), (Region::B, Facing::North)),
        ((Region::C, Facing::South), (Region::D, Facing::South)),
        ((Region::C, Facing::West), (Region::E, Facing::South)),
        ((Region::C, Facing::East), (Region::A, Facing::North)),
        ((Region::D, Facing::North), (Region::C, Facing::North)),
        ((Region::D, Facing::West), (Region::E, Facing::West)),
        ((Region::D, Facing::South), (Region::F, Facing::West)),
        ((Region::D, Facing::East), (Region::A, Facing::West)),
        ((Region::E, Facing::East), (Region::D, Facing::East)),
        ((Region::E, Facing::South), (Region::F, Facing::South)),
        ((Region::E, Facing::North), (Region::C, Facing::East)),
        ((Region::E, Facing::West), (Region::B, Facing::East)),
        ((Region::F, Facing::North), (Region::E, Facing::North)),
        ((Region::F, Facing::East), (Region::D, Facing::North)),
        ((Region::F, Facing::West), (Region::B, Facing::South)),
        ((Region::F, Facing::South), (Region::A, Facing::South)),
    ])
}

fn changed_coordinate(old: &Coord, old_facing: &Facing, new_facing: &Facing) -> Coord {
    match old_facing {
        Facing::North => match new_facing {
            Facing::North => Coord(old.0, SQUARE_SIZE - 1),
            Facing::East => Coord(0, old.0),
            _ => panic!("nonexisting change from north to {:?}", new_facing),
        },
        Facing::East => match new_facing {
            Facing::North => Coord(old.1, SQUARE_SIZE - 1),
            Facing::East => Coord(0, old.1),
            Facing::West => Coord(SQUARE_SIZE - 1, SQUARE_SIZE - old.1 - 1),
            _ => panic!("nonexisting change from east to {:?}", new_facing),
        },
        Facing::South => match new_facing {
            Facing::South => Coord(old.0, 0),
            Facing::West => Coord(SQUARE_SIZE - 1, old.0),
            _ => panic!("nonexisting change from south to {:?}", new_facing),
        },
        Facing::West => match new_facing {
            Facing::West => Coord(SQUARE_SIZE - 1, old.1),
            Facing::South => Coord(old.1, 0),
            Facing::East => Coord(0, SQUARE_SIZE - old.1 - 1),
            _ => panic!("nonexisting change from west to {:?}", new_facing),
        },
    }
}

fn region_to_map_coordinate(region: &Region, coord: &Coord) -> Coord {
    match region {
        Region::A => Coord(coord.0 + 2 * SQUARE_SIZE, coord.1),
        Region::B => Coord(coord.0 + SQUARE_SIZE, coord.1),
        Region::C => Coord(coord.0 + SQUARE_SIZE, coord.1 + SQUARE_SIZE),
        Region::D => Coord(coord.0 + SQUARE_SIZE, coord.1 + 2 * SQUARE_SIZE),
        Region::E => Coord(coord.0, coord.1 + 2 * SQUARE_SIZE),
        Region::F => Coord(coord.0, coord.1 + 3 * SQUARE_SIZE),
    }
}

struct Puzzle {
    map: HashMap<Coord, bool>,
    instructions: VecDeque<Instruction>,
    region: Region,
    position: Coord,
    facing: Facing,
    region_changes: HashMap<(Region, Facing), (Region, Facing)>,
}

impl Puzzle {
    fn walk(mut self) {
        while let Some(instruction) = self.instructions.pop_front() {
            match instruction {
                Instruction::Forward(nr) => self.forward(nr),
                Instruction::Left => self.turn_counter_clockwise(),
                Instruction::Right => self.turn_clockwise(),
            }
        }
        println!("score: {}", self.score());
    }

    fn score(&self) -> usize {
        let facing_score = match self.facing {
            Facing::North => 3,
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
        };

        let map_coord = region_to_map_coordinate(&self.region, &self.position);
        4 * (map_coord.0 + 1) + 1_000 * (map_coord.1 + 1) + facing_score
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
        for _ in 0..nr {
            let coord_option = match self.facing {
                Facing::North => {
                    (self.position.1 > 0).then(|| Coord(self.position.0, self.position.1 - 1))
                }
                Facing::East => (self.position.0 < SQUARE_SIZE - 1)
                    .then(|| Coord(self.position.0 + 1, self.position.1)),
                Facing::South => (self.position.1 < SQUARE_SIZE - 1)
                    .then(|| Coord(self.position.0, self.position.1 + 1)),
                Facing::West => {
                    (self.position.0 > 0).then(|| Coord(self.position.0 - 1, self.position.1))
                }
            };

            let candidate = if let Some(coord) = coord_option {
                (self.region, self.facing, coord)
            } else {
                self.candidate_changed_region()
            };

            let map_coord = region_to_map_coordinate(&candidate.0, &candidate.2);
            if !self.map[&map_coord] {
                return;
            }

            self.region = candidate.0;
            self.facing = candidate.1;
            self.position = candidate.2;
        }
    }

    fn candidate_changed_region(&mut self) -> (Region, Facing, Coord) {
        let change = self
            .region_changes
            .get(&(self.region, self.facing))
            .unwrap();
        let coord = changed_coordinate(&self.position, &self.facing, &change.1);

        (change.0, change.1, coord)
    }
}

fn parse() -> Puzzle {
    let lines = lines_from_file("in");
    let mut puzzle_parts = lines.split(|e| e.is_empty());

    let map = parse_map(&puzzle_parts.next().unwrap());
    let instructions = parse_instructions(&puzzle_parts.next().unwrap()[0]);

    Puzzle {
        map,
        instructions,
        region: Region::B,
        position: Coord(0, 0),
        facing: Facing::East,
        region_changes: region_changes(),
    }
}

fn parse_map(lines: &[String]) -> HashMap<Coord, bool> {
    let mut result: HashMap<Coord, bool> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let element_option: Option<bool> = match char {
                '.' => Some(true),
                '#' => Some(false),
                _ => None,
            };

            if let Some(element) = element_option {
                result.insert(Coord(x, y), element);
            }
        }
    }

    result
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
