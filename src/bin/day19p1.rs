#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    cmp,
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    ops::{Add, Sub},
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

const EXPIRE: usize = 24;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug)]
enum Robot {
    Ore(Resources),
    Clay(Resources),
    Obsidoan(Resources),
    Geode(Resources),
}

type Blueprint = Vec<Robot>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
}

fn solve() {
    let blueprints: Vec<Blueprint> = parse();

    let r = walk(&blueprints[1]);
    println!("{}", r);
}

fn walk(blueprint: &Blueprint) -> usize {
    let mut state_stack: Vec<State> = Vec::new();
    state_stack.push(State {
        minute: 0,
        robots: RobotInventory {
            ore_robot_count: 1,
            clay_robot_count: 0,
            obsidian_robot_count: 0,
        },
        funds: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
        },
        geode_end_count: 0,
    });

    let mut result = 0;
    let mut visited: HashSet<State> = HashSet::new();
    while !state_stack.is_empty() {
        let state = state_stack.pop().unwrap();

        if !visited.insert(state.clone()) {
            continue;
        }

        result = cmp::max(result, state.geode_end_count);

        if state.minute < EXPIRE {
            let mut could_buy_geo_robot = false;
            for robot in blueprint {
                if let Some(next_state) = buy_robot(&state, robot) {
                    if let Robot::Geode(_) = robot {
                        could_buy_geo_robot = true;
                    }

                    state_stack.push(next_state);
                }
            }

            if !could_buy_geo_robot {
                // don't buy but save
                state_stack.push(update_state(state.clone(), &state.robots));
            }
        }
    }

    result
}

fn buy_robot(state: &State, robot: &Robot) -> Option<State> {
    let mut result = state.clone();

    match robot {
        Robot::Ore(price) => {
            if let Some(left) = spend_funds(&state.funds, &price) {
                result.funds = left;
                result.robots.ore_robot_count += 1;
            } else {
                return None;
            }
        }
        Robot::Clay(price) => {
            if let Some(left) = spend_funds(&state.funds, &price) {
                result.funds = left;
                result.robots.clay_robot_count += 1;
            } else {
                return None;
            }
        }
        Robot::Obsidoan(price) => {
            if let Some(left) = spend_funds(&state.funds, &price) {
                result.funds = left;
                result.robots.obsidian_robot_count += 1;
            } else {
                return None;
            }
        }
        Robot::Geode(price) => {
            if let Some(left) = spend_funds(&state.funds, &price) {
                result.funds = left;
                result.geode_end_count += EXPIRE - state.minute - 1;
            } else {
                return None;
            }
        }
    }

    result = update_state(result, &state.robots);
    Some(result)
}

fn update_state(mut state_to_update: State, inventory: &RobotInventory) -> State {
    state_to_update.funds.ore += inventory.ore_robot_count;
    state_to_update.funds.clay += inventory.clay_robot_count;
    state_to_update.funds.obsidian += inventory.obsidian_robot_count;
    state_to_update.minute += 1;

    state_to_update
}

fn spend_funds(funds: &Resources, price: &Resources) -> Option<Resources> {
    if price.ore <= funds.ore && price.clay <= funds.clay && price.obsidian <= funds.obsidian {
        return Some(funds - price);
    }

    None
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

        blueprints.push(vec![
            Robot::Ore(ore_robot),
            Robot::Clay(clay_robot),
            Robot::Obsidoan(obsidian_robot),
            Robot::Geode(geode_robot),
        ]);
    }

    blueprints
}
