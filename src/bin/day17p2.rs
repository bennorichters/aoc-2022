#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    cmp,
    collections::{HashMap, HashSet},
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

const BLOCK_SIZE: usize = 4;

const BLOCK0: [[bool; BLOCK_SIZE]; BLOCK_SIZE] = [
    [true, true, true, true],
    [false, false, false, false],
    [false, false, false, false],
    [false, false, false, false],
];

const BLOCK1: [[bool; BLOCK_SIZE]; BLOCK_SIZE] = [
    [false, true, false, false],
    [true, true, true, false],
    [false, true, false, false],
    [false, false, false, false],
];

const BLOCK2: [[bool; BLOCK_SIZE]; BLOCK_SIZE] = [
    [true, true, true, false],
    [false, false, true, false],
    [false, false, true, false],
    [false, false, false, false],
];

const BLOCK3: [[bool; BLOCK_SIZE]; BLOCK_SIZE] = [
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
    [true, false, false, false],
];

const BLOCK4: [[bool; BLOCK_SIZE]; BLOCK_SIZE] = [
    [true, true, false, false],
    [true, true, false, false],
    [false, false, false, false],
    [false, false, false, false],
];

const BLOCK_COUNT: usize = 5;
const BLOCKS: [[[bool; BLOCK_SIZE]; BLOCK_SIZE]; BLOCK_COUNT] =
    [BLOCK0, BLOCK1, BLOCK2, BLOCK3, BLOCK4];
const BLOCKS_HEIGHT: [usize; BLOCK_COUNT] = [1, 3, 3, 4, 2];
const BLOCKS_WIDTH: [usize; BLOCK_COUNT] = [4, 3, 3, 1, 2];

const CHAMBER_WIDTH: usize = 7;
const START_X: usize = 2;
const START_Y: usize = 4;
const SPACE_ABOVE: usize = 3;

const ROUNDS: usize = 100_000;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct BlockJetIndices(usize, usize);

fn solve() {
    let lines = lines_from_file("in");
    let jet: Vec<bool> = lines[0].chars().map(|c| c == '>').collect();
    let mut jet_cycle = (0..jet.len()).cycle();

    let mut chamber: Vec<[bool; CHAMBER_WIDTH]> = Vec::new();
    chamber.push([true; CHAMBER_WIDTH]);

    let mut y = START_Y;
    let mut blocks_cycle = (0..BLOCKS.len()).cycle();
    let mut tower_height = 0;

    let mut jet_index: usize = 0;
    let mut detect: HashMap<BlockJetIndices, usize> = HashMap::new();
    for round in 0..ROUNDS {
        for _ in chamber.len()..(y + BLOCK_SIZE) {
            chamber.push([false; CHAMBER_WIDTH]);
        }

        let block_index = blocks_cycle.next().unwrap();


        // Round 51465 is magic :)
        // it has height 80761
        // after 1715 rounds we arrive at the same block_index (0) with the same jet_index (10031)
        // the next time with this combination the height is 2690 higher
        // this repeats (1_000_000_000_000 - 51465) / 1715 = 583090349 times before we arrive, so
        // 583090349 + 2690 + 80761 is the answer
        if block_index == 0 {
            let bji = BlockJetIndices(block_index, jet_index);
            if let Some(height) = detect.get(&bji) {
                println!(
                    "{}, {:?}, {}, {}, {}",
                    round,
                    bji,
                    tower_height,
                    height,
                    tower_height - height
                );
            }
            detect.insert(bji, tower_height);
        }

        let block = &BLOCKS[block_index];
        let mut x = START_X;
        let max_x = CHAMBER_WIDTH - BLOCKS_WIDTH[block_index];
        loop {
            jet_index = jet_cycle.next().unwrap();
            let right = jet[jet_index];

            if right && x < max_x && !overlap(&chamber, block, x + 1, y) {
                x += 1;
            } else if !right && x > 0 && !overlap(&chamber, block, x - 1, y) {
                x -= 1;
            };

            if overlap(&chamber, &BLOCKS[block_index], x, y - 1) {
                break;
            }

            y -= 1;
        }

        add_block(&mut chamber, block, x, y);
        tower_height = cmp::max(tower_height, y - 1 + BLOCKS_HEIGHT[block_index]);
        y = tower_height + SPACE_ABOVE + 1;
    }

    println!("{}", tower_height);
}

fn add_block(
    chamber: &mut Vec<[bool; CHAMBER_WIDTH]>,
    block: &'static [[bool; BLOCK_SIZE]; BLOCK_SIZE],
    x: usize,
    y: usize,
) {
    for yy in 0..BLOCK_SIZE {
        for xx in 0..BLOCK_SIZE {
            if x + xx < CHAMBER_WIDTH {
                chamber[y + yy][x + xx] |= block[yy][xx];
            }
        }
    }
}

fn overlap(
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
    block: &'static [[bool; BLOCK_SIZE]; BLOCK_SIZE],
    x: usize,
    y: usize,
) -> bool {
    for test_y in 0..BLOCK_SIZE {
        for test_x in 0..cmp::min(BLOCK_SIZE, CHAMBER_WIDTH - x) {
            if chamber[y + test_y][x + test_x] && block[test_y][test_x] {
                return true;
            }
        }
    }

    false
}

fn print_chamber(
    chamber: &Vec<[bool; CHAMBER_WIDTH]>,
    block: &'static [[bool; BLOCK_SIZE]; BLOCK_SIZE],
    block_x: usize,
    block_y: usize,
) {
    for y in (0..chamber.len()).rev() {
        for x in 0..CHAMBER_WIDTH {
            print!(
                "{}",
                if chamber[y][x] {
                    "#"
                } else if (block_x..(block_x + BLOCK_SIZE)).contains(&x)
                    && (block_y..(block_y + BLOCK_SIZE)).contains(&y)
                    && block[y - block_y][x - block_x]
                {
                    "@"
                } else {
                    "."
                }
            );
        }
        println!();
    }
}
