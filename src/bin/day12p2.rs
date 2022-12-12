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
        .map(|e| e.expect("could not parse line"))
        .collect()
}

fn main() {
    solve();
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coord(usize, usize);

fn solve() {
    let lines = lines_from_file("in");

    let mut map: HashMap<Coord, u8> = HashMap::new();
    let mut e = Coord(0, 0);

    let width = lines[0].len();
    let height = lines.len();

    let mut zeros: Vec<Coord> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let c = lines[y].chars().nth(x).unwrap();
            if c == 'S' {
                map.insert(Coord(x, y), 0);
                zeros.push(Coord(x, y));
            } else if c == 'E' {
                e = Coord(x, y);
                map.insert(Coord(x, y), 25);
            } else {
                let elevation = lines[y].as_bytes().iter().nth(x).unwrap() - b'a';
                map.insert(Coord(x, y), elevation);

                if elevation == 0 {
                    zeros.push(Coord(x, y));
                }
            }
        }
    }

    let mut puzzle = Puzzle {
        map,
        width,
        height,
        e,
    };

    puzzle.start(zeros);
}

#[derive(Debug)]
struct Puzzle {
    map: HashMap<Coord, u8>,
    width: usize,
    height: usize,
    e: Coord,
}

impl Puzzle {
    fn options(&self, current: &Coord) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();

        if current.0 > 0 {
            result.push(Coord(current.0 - 1, current.1));
        }
        if current.0 < self.width - 1 {
            result.push(Coord(current.0 + 1, current.1));
        }
        if current.1 > 0 {
            result.push(Coord(current.0, current.1 - 1));
        }
        if current.1 < self.height - 1 {
            result.push(Coord(current.0, current.1 + 1));
        }

        result
    }

    fn start(&mut self, zeros: Vec<Coord>) {
        let mut result = 1000;
        for z in zeros {
            let path = self.walk(&z);
            if path.len() > 0 {
                result = cmp::min(result, path.len());
            }
        }
        println!("{}", result);
    }

    fn walk(&mut self, current: &Coord) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();
        let mut state_stack: Vec<State> = Vec::new();

        let mut visited: HashMap<Coord, usize> = HashMap::new();
        state_stack.push(State {
            position: current.clone(),
            elevation: 0,
            path: Vec::new(),
        });

        while !state_stack.is_empty() {
            state_stack.sort_by(|a, b| b.path.len().cmp(&a.path.len()));
            let state = state_stack.pop().unwrap();

            if !visited.contains_key(&state.position)
                || *visited.get(&state.position).unwrap() > state.path.len()
            {
                if state.position == self.e {
                    result = state.path.clone();
                }
                visited.insert(state.position.clone(), state.path.len());

                let options = self.options(&state.position);
                for op in &options {
                    let elevation = self.map.get(&op).unwrap();
                    if elevation <= &(state.elevation + 1) {
                        let mut extra_path = state.path.clone();
                        extra_path.push(op.clone());

                        state_stack.push(State {
                            position: op.clone(),
                            elevation: *elevation,
                            path: extra_path,
                        });
                    }
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct State {
    position: Coord,
    elevation: u8,
    path: Vec<Coord>,
}
