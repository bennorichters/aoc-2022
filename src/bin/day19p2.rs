// #![allow(dead_code)]
// #![allow(unused_variables)]

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

const EXPIRE: usize = 32;

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

type Blueprint = Vec<Resources>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    minute: usize,
    funds: Resources,
    robots: Vec<usize>,
    geode_end_count: usize,
}

const ROBOT_ORE: usize = 0;
const ROBOT_CLAY: usize = 1;
const ROBOT_OBSIDIAN: usize = 2;
const ROBOT_GEO: usize = 3;

fn solve() {
    let blueprints: Vec<Blueprint> = parse();

    let mut result = 1;
    for i in 0..3 {
        println!("{}", i);
        let geo = walk(&blueprints[i]);
        result *= geo;
        println!("------------>  {}, {}", result, geo);
    }
    println!("{}", result);
}

fn walk(blueprint: &Blueprint) -> usize {
    let mut state_stack: Vec<State> = Vec::new();
    state_stack.push(State {
        minute: 0,
        robots: vec![1, 0, 0],
        funds: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
        },
        geode_end_count: 0,
    });

    let max_costs = calc_max_costs(blueprint);

    let mut result = 0;
    let mut visited: HashSet<State> = HashSet::new();
    while !state_stack.is_empty() {
        state_stack.sort_by(|a, b| a.geode_end_count.cmp(&b.geode_end_count));
        let state = state_stack.pop().unwrap();
        // if state.geode_end_count > 5 {
        //     for s in state_stack {
        //         println!("------------> {:?}", s);
        //     }
        //     return 42;
        // }

        if !visited.insert(state.clone()) {
            continue;
        }

        if state.geode_end_count > result {
            println!("{:?}", state);
        }
        result = cmp::max(result, state.geode_end_count);

        let minutes_remain = EXPIRE - state.minute;
        if minutes_remain > 0 && potential(state.geode_end_count, state.minute) > result {
            for (robot_index, robot_costs) in blueprint.iter().enumerate() {
                if !enough_robots(&state, &max_costs, robot_index) {
                    if let Some(funds_remain) = spend_funds(&state.funds, robot_costs) {
                        let mut next_state = next_state(&state, &funds_remain);
                        if robot_index == ROBOT_GEO {
                            next_state.geode_end_count += minutes_remain - 1;
                        } else {
                            next_state.robots[robot_index] += 1;
                        }
                        state_stack.push(next_state);
                    }
                }
            }

            if can_save(&state, &max_costs) {
                state_stack.push(next_state(&state, &state.funds));
            }
        }
    }

    result
}

fn can_save(state: &State, max_costs: &Resources) -> bool {
    state.funds.ore < max_costs.ore
        || (state.funds.clay < max_costs.clay && state.robots[ROBOT_CLAY] > 0)
        || (state.funds.obsidian < max_costs.obsidian && state.robots[ROBOT_OBSIDIAN] > 0)
}

fn potential(current: usize, minute: usize) -> usize {
    let minutes_remain = EXPIRE - minute;
    current + minutes_remain * (minutes_remain + 1) / 2
}

fn enough_robots(state: &State, max_costs: &Resources, robot_index: usize) -> bool {
    if robot_index == ROBOT_GEO {
        return false;
    }

    let minutes_remain = EXPIRE - state.minute;

    let stock = match robot_index {
        ROBOT_ORE => state.funds.ore,
        ROBOT_CLAY => state.funds.clay,
        _ => state.funds.obsidian,
    };

    let needed = match robot_index {
        ROBOT_ORE => max_costs.ore,
        ROBOT_CLAY => max_costs.clay,
        _ => max_costs.obsidian,
    };

    let robot_count = state.robots[robot_index];

    robot_count * minutes_remain + stock >= needed * minutes_remain
}

fn next_state(state: &State, funds_remain: &Resources) -> State {
    let next_funds = Resources {
        ore: funds_remain.ore + state.robots[ROBOT_ORE],
        clay: funds_remain.clay + state.robots[ROBOT_CLAY],
        obsidian: funds_remain.obsidian + state.robots[ROBOT_OBSIDIAN],
    };

    State {
        minute: state.minute + 1,
        funds: next_funds,
        robots: state.robots.clone(),
        geode_end_count: state.geode_end_count,
    }
}

fn calc_max_costs(blueprint: &Blueprint) -> Resources {
    let mut result = Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
    };

    for robot in blueprint {
        result.ore = cmp::max(result.ore, robot.ore);
        result.clay = cmp::max(result.clay, robot.clay);
        result.obsidian = cmp::max(result.obsidian, robot.obsidian);
    }

    result
}

fn spend_funds(funds: &Resources, price: &Resources) -> Option<Resources> {
    if price.ore <= funds.ore && price.clay <= funds.clay && price.obsidian <= funds.obsidian {
        return Some(funds - price);
    }

    None
}

fn parse() -> Vec<Blueprint> {
    let lines = lines_from_file("in");

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

        blueprints.push(vec![ore_robot, clay_robot, obsidian_robot, geode_robot]);
    }

    blueprints
}
