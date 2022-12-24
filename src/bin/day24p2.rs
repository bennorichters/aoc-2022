use std::{
    cmp,
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
    let parsed = parse("in");
    let width = parsed.0;
    let height = parsed.1;
    let map = parsed.2;

    let puzzle = Puzzle {
        width,
        height,
        cycles: width * height,
        destination: Coord(width - 1, height - 1),
    };
    puzzle.solve(map);
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(usize, usize);

impl Coord {
    fn neighbour(&self, direction: &Direction, width: usize, height: usize) -> Option<Coord> {
        match direction {
            Direction::North => {
                if self.1 == 0 {
                    None
                } else {
                    Some(Coord(self.0, self.1 - 1))
                }
            }
            Direction::East => {
                if self.0 == width - 1 {
                    None
                } else {
                    Some(Coord(self.0 + 1, self.1))
                }
            }
            Direction::South => {
                if self.1 == height - 1 {
                    None
                } else {
                    Some(Coord(self.0, self.1 + 1))
                }
            }
            Direction::West => {
                if self.0 == 0 {
                    None
                } else {
                    Some(Coord(self.0 - 1, self.1))
                }
            }
        }
    }

    fn wrap_around_neighbour(&self, direction: &Direction, width: usize, height: usize) -> Coord {
        if let Some(candidate) = self.neighbour(direction, width, height) {
            candidate
        } else {
            match direction {
                Direction::North => Coord(self.0, height - 1),
                Direction::East => Coord(0, self.1),
                Direction::South => Coord(self.0, 0),
                Direction::West => Coord(width - 1, self.1),
            }
        }
    }

    fn manhatten_distance(&self, other: &Coord) -> usize {
        let diff_x = cmp::max(self.0, other.0) - cmp::min(self.0, other.0);
        let diff_y = cmp::max(self.1, other.1) - cmp::min(self.1, other.1);

        diff_x + diff_y
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

struct Puzzle {
    width: usize,
    height: usize,
    cycles: usize,
    destination: Coord,
}

impl Puzzle {
    fn solve(&self, start_map: HashMap<Coord, Vec<Direction>>) {
        let all_maps = self.all_maps(start_map);

        let r1 = self.walk(&all_maps, 0, &Coord(0, 0), &self.destination);
        println!("------------ {r1}");
        let r2 = self.walk(
            &all_maps,
            r1,
            &Coord(self.destination.0, self.destination.1 - 1),
            &Coord(0, 0),
        );
        println!("------------ {r2}");
        let r3 = self.walk(&all_maps, r1 + r2, &Coord(0, 0), &self.destination);
        println!("------------ {r3}");
        println!("Result: {:?}", r1 + r2 + r3);
    }

    fn walk(
        &self,
        all_maps: &Vec<HashMap<Coord, Vec<Direction>>>,
        start_map_index: usize,
        start: &Coord,
        goal: &Coord,
    ) -> usize {
        let mut state_stack: Vec<State> = Vec::new();
        state_stack.push(State {
            position_option: None,
            minute: 0,
        });

        let mut result_option: Option<usize> = None;
        let mut visited: HashMap<(Option<Coord>, usize), usize> = HashMap::new();
        while !state_stack.is_empty() {
            let best = self.best_index(goal, &state_stack);
            let state = state_stack.remove(best);

            let visited_key = (state.position_option.clone(), state.minute % self.cycles);
            let prev_option = visited.get(&visited_key);
            if matches!(prev_option, Some(previous) if previous <= &state.minute) {
                continue;
            }
            visited.insert(visited_key, state.minute);

            if matches!(result_option, Some(result) if result <= state.minute) {
                continue;
            }

            if matches!(&state.position_option, Some(position) if position == goal) {
                if let Some(result) = result_option {
                    result_option = Some(cmp::min(result, state.minute));
                } else {
                    result_option = Some(state.minute);
                }

                println!("Best until now: {:?}", result_option.unwrap() + 1);
            }

            let ps = self.possibilities(&state, &start);
            for p in ps {
                let next_state_option = self.next_state(
                    state.minute,
                    &all_maps[(state.minute + start_map_index + 1) % self.cycles],
                    p,
                );
                if let Some(next_state) = next_state_option {
                    state_stack.push(next_state);
                }
            }
        }

        result_option.unwrap() + 1
    }

    fn best_index(&self, goal: &Coord, state_stack: &Vec<State>) -> usize {
        state_stack
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                if let (Some(position_a), Some(position_b)) =
                    (&a.position_option, &b.position_option)
                {
                    let mda = position_a.manhatten_distance(goal);
                    let mdb = position_b.manhatten_distance(goal);
                    mda.cmp(&mdb)
                } else if a.position_option.is_some() {
                    cmp::Ordering::Less
                } else if b.position_option.is_some() {
                    cmp::Ordering::Greater
                } else {
                    a.minute.cmp(&b.minute)
                }
            })
            .map(|(index, _)| index)
            .unwrap()
    }

    fn possibilities(&self, state: &State, start: &Coord) -> Vec<Option<Coord>> {
        let mut result: Vec<Option<Coord>> = Vec::new();
        if let Some(position) = &state.position_option {
            for direction in ALL_DIRECTIONS {
                let neighbour_option = position.neighbour(&direction, self.width, self.height);
                if neighbour_option.is_some() {
                    result.push(neighbour_option);
                }
            }
            result.push(state.position_option.clone());
        } else {
            result.push(None);
            result.push(Some(start.clone()));
        }

        result
    }

    fn next_state(
        &self,
        minute: usize,
        next_map: &HashMap<Coord, Vec<Direction>>,
        next_position_option: Option<Coord>,
    ) -> Option<State> {
        if matches!(&next_position_option, Some(next_position) if next_map.contains_key(&next_position))
        {
            return None;
        }

        Some(State {
            position_option: next_position_option,
            minute: minute + 1,
        })
    }

    fn all_maps(
        &self,
        start_map: HashMap<Coord, Vec<Direction>>,
    ) -> Vec<HashMap<Coord, Vec<Direction>>> {
        let mut result: Vec<HashMap<Coord, Vec<Direction>>> = Vec::new();

        let mut prev = start_map.clone();
        result.push(start_map);
        for _ in 0..self.cycles {
            let mut next_map: HashMap<Coord, Vec<Direction>> = HashMap::new();
            for coord in prev.keys() {
                let blizzards = prev.get(coord);
                if let Some(blizzard) = blizzards {
                    for direction in blizzard {
                        let next_blizzard_coord =
                            coord.wrap_around_neighbour(direction, self.width, self.height);

                        next_map
                            .entry(next_blizzard_coord.clone())
                            .or_insert(Vec::new());

                        let directions = next_map.get_mut(&next_blizzard_coord).unwrap();
                        directions.push(direction.clone());
                    }
                }
            }
            prev = next_map.clone();
            result.push(next_map);
        }

        result
    }

    fn _print_state_map(
        &self,
        map: &HashMap<Coord, Vec<Direction>>,
        position_option: &Option<Coord>,
    ) {
        for y in 0..self.height {
            for x in 0..self.width {
                let blizzards_option = map.get(&Coord(x, y));
                let c = if let Some(blizzards) = blizzards_option {
                    if blizzards.len() == 0 {
                        panic!("cant have empty vec of blizzards")
                    } else if blizzards.len() == 1 {
                        match blizzards[0] {
                            Direction::North => "^",
                            Direction::East => ">",
                            Direction::South => "v",
                            Direction::West => "<",
                        }
                        .to_owned()
                    } else {
                        blizzards.len().to_string()
                    }
                } else if let Some(me) = position_option {
                    if me == &Coord(x, y) {
                        "E".to_owned()
                    } else {
                        ".".to_owned()
                    }
                } else {
                    "-".to_string()
                };

                print!("{c}");
            }
            println!()
        }
    }
}

#[derive(Debug)]
struct State {
    position_option: Option<Coord>,
    minute: usize,
}

fn parse(input: &str) -> (usize, usize, HashMap<Coord, Vec<Direction>>) {
    let lines = lines_from_file(input);

    let width = lines[0].len();
    let height = lines.len();

    let mut map: HashMap<Coord, Vec<Direction>> = HashMap::new();
    for (y, line) in lines[1..(height - 1)].iter().enumerate() {
        for (x, char) in line[1..(width - 1)].chars().enumerate() {
            let blizzards_option = match char {
                '.' => None,
                '^' => Some(vec![Direction::North]),
                '>' => Some(vec![Direction::East]),
                'v' => Some(vec![Direction::South]),
                '<' => Some(vec![Direction::West]),
                _ => panic!("parse error"),
            };

            if let Some(blizzards) = blizzards_option {
                map.insert(Coord(x, y), blizzards);
            }
        }
    }

    (width - 2, height - 2, map)
}
