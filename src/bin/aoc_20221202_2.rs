use std::{
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
    let lines = lines_from_file("in");

    let mut score = 0;
    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();

        let other = s[0];
        let you = s[1];

        // Rock 1
        // Paper 2
        // Scissors 3
        match other {
            // Rock
            "A" => match you {
                "X" => score += 3 + 0,
                "Y" => score += 1 + 3,
                "Z" => score += 2 + 6,
                _ => panic!(""),
            },
            // Paper
            "B" => match you {
                "X" => score += 1 + 0,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => panic!(""),
            },

            // Scissors
            "C" => match you {
                "X" => score += 2 + 0,
                "Y" => score += 3 + 3,
                "Z" => score += 1 + 6,
                _ => panic!(""),
            },

            _ => panic!(""),
        }

    }
    println!("{:?}", score);
}

