// #![allow(dead_code)]
// #![allow(unused_variables)]

use std::{
    cmp,
    collections::HashMap,
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

#[derive(Debug)]
struct State {
    minute: usize,
    current_valve: usize,
    released: u64,
    closed_valves: Vec<usize>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct StartEnd {
    start: usize,
    end: usize,
}

fn solve() {
    let valves = parse();

    let closed_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|&(_, v)| v.0 > 0)
        .map(|e| e.0)
        .collect();

    let flows: Vec<u64> = valves.iter().map(|v| v.0).collect();

    let mut distances: HashMap<StartEnd, usize> = HashMap::new();
    let mut start_valves = closed_valves.clone();
    start_valves.push(0);
    for start in &start_valves {
        for destination in &closed_valves {
            if start != destination {
                let dist = find_distance(*start, *destination, &valves);
                let key = StartEnd {
                    start: *cmp::min(start, destination),
                    end: *cmp::max(start, destination),
                };
                distances.insert(key, dist);
            }
        }
    }

    walk(closed_valves, flows, distances);
}

fn find_distance(start: usize, destination: usize, valves: &Vec<(u64, Vec<usize>)>) -> usize {
    let mut state_stack: Vec<(usize, usize)> = Vec::new();
    state_stack.push((start, 0));

    let mut result_option: Option<usize> = None;
    let mut visited: Vec<Option<usize>> = vec![None; valves.len()];
    while !state_stack.is_empty() {
        let state = state_stack.pop().unwrap();

        if state.0 == destination {
            if let Some(result) = result_option {
                if state.1 < result {
                    result_option = Some(state.1);
                }
            } else {
                result_option = Some(state.1);
            }

            continue;
        }

        if let Some(previous) = visited[state.0] {
            if previous <= state.1 {
                continue;
            }
        }
        visited[state.0] = Some(state.1);

        for next in &valves[state.0].1 {
            state_stack.push((*next, state.1 + 1));
        }
    }

    result_option.unwrap()
}

fn walk(closed_valves: Vec<usize>, flows: Vec<u64>, distances: HashMap<StartEnd, usize>) {
    let mut state_stack: Vec<State> = vec![State {
        minute: 0,
        current_valve: 0,
        released: 0,
        closed_valves,
    }];

    let mut visited = vec![064; 30];
    let mut result = 0;
    while !state_stack.is_empty() {
        state_stack.sort_by(|a, b| a.minute.cmp(&b.minute));
        let state = state_stack.pop().unwrap();

        if state.minute <= 30 {
            result = cmp::max(state.released, result);
        }

        if state.minute >= 30 {
            continue;
        }

        if state.released > visited[state.minute] {
            visited[state.minute] = state.released;
        } else if potential(state.released, &state.closed_valves, state.minute, &flows)
            <= visited[state.minute]
        {
            continue;
        }

        if state.closed_valves.contains(&state.current_valve) {
            state_stack.push(open_valve_state(state, &flows));
        } else {
            state_stack.append(&mut next_closed_valve_states(state, &distances));
        }
    }

    println!("{:?}", result);
}

fn next_closed_valve_states(current: State, distances: &HashMap<StartEnd, usize>) -> Vec<State> {
    let mut result: Vec<State> = Vec::new();
    for next_closed in &current.closed_valves {
        if *next_closed != current.current_valve {
            let key = StartEnd {
                start: cmp::min(current.current_valve, *next_closed),
                end: cmp::max(current.current_valve, *next_closed),
            };
            let distance = distances.get(&key).unwrap();

            result.push(State {
                minute: current.minute + distance,
                current_valve: *next_closed,
                released: current.released,
                closed_valves: current.closed_valves.clone(),
            });
        }
    }

    result
}

fn open_valve_state(current: State, flows: &Vec<u64>) -> State {
    let stack_released =
        current.released + ((30 - current.minute - 1) as u64) * flows[current.current_valve];

    let stack_closed_valves: Vec<usize> = current
        .closed_valves
        .iter()
        .filter(|&&e| e != current.current_valve)
        .cloned()
        .collect();

    State {
        minute: current.minute + 1,
        current_valve: current.current_valve,
        released: stack_released,
        closed_valves: stack_closed_valves,
    }
}

fn potential(released: u64, closed_valves: &Vec<usize>, minute: usize, flows: &Vec<u64>) -> u64 {
    let mut result = released;
    for c in closed_valves {
        result += (30 - minute - 1) as u64 * flows[*c];
    }

    result
}

fn parse() -> Vec<(u64, Vec<usize>)> {
    let lines = lines_from_file("in");

    let mut raw: Vec<(String, u64, Vec<String>)> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let valve = parts[1].to_owned();
        let connects: Vec<String> = parts[9..]
            .iter()
            .cloned()
            .map(|p| p.replace(",", "").to_owned())
            .collect();

        let re = Regex::new(r"\d+").unwrap();
        let cap = re.captures(&line).unwrap();
        let flow_rate = cap.get(0).unwrap().as_str().parse::<u64>().unwrap();

        raw.push((valve, flow_rate, connects));
    }

    raw.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result: Vec<(u64, Vec<usize>)> = Vec::new();
    for raw_element in &raw {
        let mut connecting_indices: Vec<usize> = Vec::new();
        for connecting_valve in &raw_element.2 {
            let i = raw.iter().position(|e| &e.0 == connecting_valve).unwrap();
            connecting_indices.push(i);
        }

        connecting_indices.sort();
        result.push((raw_element.1, connecting_indices));
    }

    result
}
