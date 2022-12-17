use std::{
    cmp,
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

fn solve() {
    let valves = parse();

    let closed_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|&(_, v)| v.0 > 0)
        .map(|e| e.0)
        .collect();

    let mut state_stack: Vec<State> = vec![State {
        minute: 0,
        current_valve: 0,
        released: 0,
        closed_valves,
    }];

    let mut visited = vec![064; 30];
    let mut result_option: Option<u64> = None;
    while !state_stack.is_empty() {
        let state = state_stack.pop().unwrap();

        result_option = if let Some(result) = result_option {
            Some(cmp::max(state.released, result))
        } else {
            Some(state.released)
        };

        if state.minute < 30 {
            if state.released > visited[state.minute] {
                visited[state.minute] = state.released;
            } else {
                let potential =
                    calc_potential(state.released, &state.closed_valves, state.minute, &valves);
                if potential <= visited[state.minute] {
                    continue;
                }
            }

            if state.closed_valves.contains(&state.current_valve) {
                let stack_released = state.released
                    + ((30 - state.minute - 1) as u64) * valves[state.current_valve].0;

                let stack_closed_valves: Vec<usize> = state
                    .closed_valves
                    .iter()
                    .filter(|&&e| e != state.current_valve)
                    .cloned()
                    .collect();

                state_stack.push(State {
                    minute: state.minute + 1,
                    current_valve: state.current_valve,
                    released: stack_released,
                    closed_valves: stack_closed_valves,
                });
            }

            for next in &valves[state.current_valve].1 {
                state_stack.push(State {
                    minute: state.minute + 1,
                    current_valve: *next,
                    released: state.released,
                    closed_valves: state.closed_valves.clone(),
                });
            }
        }
    }

    println!("{:?}", result_option);
}

fn calc_potential(
    released: u64,
    closed_valves: &Vec<usize>,
    minute: usize,
    valves: &Vec<(u64, Vec<usize>)>,
) -> u64 {
    let mut result = released;
    for c in closed_valves {
        result += (30 - minute - 1) as u64 * valves[*c].0;
    }

    result
}

#[derive(Debug)]
struct State {
    minute: usize,
    current_valve: usize,
    released: u64,
    closed_valves: Vec<usize>,
}

fn parse() -> Vec<(u64, Vec<usize>)> {
    let lines = lines_from_file("tin");

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
