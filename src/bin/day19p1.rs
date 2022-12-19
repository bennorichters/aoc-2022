#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, ops::{Sub, Add},
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

#[derive(Clone, Debug)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl<'a, 'b> Add<&'b Resources> for &'a Resources {
    type Output = Resources;

    fn add(self, other: &'b Resources) -> Self::Output {
        Resources {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

impl<'a, 'b> Sub<&'b Resources> for &'a Resources {
    type Output = Resources;

    fn sub(self, other: &'b Resources) -> Self::Output {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
        }
    }
}

// #[derive(Clone, Debug)]
// enum Robot {
//     Ore(Resources),
//     Clay(Resources),
//     Obsidoan(Resources),
//     Geode(Resources),
// }

#[derive(Debug)]
struct Blueprint {
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Blueprint {
    fn buy_ore_robot(&self, funds: &Resources) -> Option<Resources> {
        self.buy(funds, &self.ore_robot)
    }

    fn buy_clay_robot(&self, funds: &Resources) -> Option<Resources> {
        self.buy(funds, &self.clay_robot)
    }

    fn buy_obsidian_robot(&self, funds: &Resources) -> Option<Resources> {
        self.buy(funds, &self.obsidian_robot)
    }

    fn buy_geode_robot(&self, funds: &Resources) -> Option<Resources> {
        self.buy(funds, &self.geode_robot)
    }

    fn buy(&self, funds: &Resources, price: &Resources) -> Option<Resources> {
        if funds.ore >= price.ore && funds.clay >= price.clay && funds.obsidian >= price.obsidian {
            return Some(funds - price);
        }

        None
    }
}

#[derive(Clone, Debug)]
struct State {
    minute: usize,
    funds: Resources,
    robots: RobotInventory,
    geode_end_count: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct RobotInventory {
    ore_robot_count: usize,
    clay_robot_count: usize,
    obsidian_robot_count: usize,
    geo_robot_count: usize,
}

fn solve() {
    let blueprints: Vec<Blueprint> = parse();
}

fn walk(blueprint: &Blueprint) -> usize {
    let mut state_stack: Vec<State> = Vec::new();
    state_stack.push(State {
        minute: 0,
        robots: RobotInventory {
            ore_robot_count: 1,
            clay_robot_count: 0,
            obsidian_robot_count: 0,
            geo_robot_count: 0,
        },
        funds: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
        },
        geode_end_count: 0,
    });

    while !state_stack.is_empty() {
        let state = state_stack.pop().unwrap();

        if let Some(funds_left) = blueprint.buy_ore_robot(&state.funds) {
            let mut next_state = next_state(&state, funds_left);
            next_state.robots.ore_robot_count += 1;
            state_stack.push(next_state);
        }

        if let Some(funds_left) = blueprint.buy_clay_robot(&state.funds) {
            let mut next_state = next_state(&state, funds_left);
            next_state.robots.clay_robot_count += 1;
            state_stack.push(next_state);
        }

        if let Some(funds_left) = blueprint.buy_obsidian_robot(&state.funds) {
            let mut next_state = next_state(&state, funds_left);
            next_state.robots.obsidian_robot_count += 1;
            state_stack.push(next_state);
        }

        if let Some(funds_left) = blueprint.buy_geode_robot(&state.funds) {
            let mut next_state = next_state(&state, funds_left);
            next_state.robots.geo_robot_count += 1;
            state_stack.push(next_state);
        }
    }

    0
}

fn next_state(state: &State, funds_left: Resources) -> State {
    let mut result = state.clone();
    result.minute += 1;
    result.funds = funds_left;

    result
}

fn parse() -> Vec<Blueprint> {
    let lines = lines_from_file("tin");

    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(".").collect();

        let re = Regex::new(r"costs\s(\d+)\sore").unwrap();
        let caps = re.captures(&parts[0]).unwrap();
        let ore: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let ore_robot = Resources {
            ore,
            clay: 0,
            obsidian: 0,
        };

        let re = Regex::new(r"costs\s(\d+)\sore").unwrap();
        let caps = re.captures(&parts[1]).unwrap();
        let ore: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let clay_robot = Resources {
            ore,
            clay: 0,
            obsidian: 0,
        };

        let re = Regex::new(r"costs\s(\d+)\sore\sand\s(\d+)\sclay").unwrap();
        let caps = re.captures(&parts[2]).unwrap();
        let ore: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let clay: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let obsidian_robot = Resources {
            ore,
            clay,
            obsidian: 0,
        };

        let re = Regex::new(r"costs\s(\d+)\sore\sand\s(\d+)\sobsidian").unwrap();
        let caps = re.captures(&parts[3]).unwrap();
        let ore: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let obsidian: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let geode_robot = Resources {
            ore,
            clay: 0,
            obsidian,
        };

        blueprints.push(Blueprint {
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        });
    }

    blueprints
}
