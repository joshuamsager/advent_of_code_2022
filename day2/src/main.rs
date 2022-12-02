use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Please specify a file to read");

    let filepath: &str = &args[1];
    println!("Score: {}", part_one(filepath));
    println!("Score: {}", part_two(filepath));
}

fn part_one(filepath: &str) -> i32 {
    let game_scheme = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 6), ("Z", 0)])),
        ("B", HashMap::from([("X", 0), ("Y", 3), ("Z", 6)])),
        ("C", HashMap::from([("X", 6), ("Y", 0), ("Z", 3)])),
    ]);

    let score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let uline = line.unwrap();
        let splitted: Vec<&str> = uline.trim().split(" ").collect();

        let (player1, player2) = (splitted[0], splitted[1]);
        let points = game_scheme[player1][player2] + score[player2];

        total_score += points;
    }

    return total_score;
}

fn part_two(filepath: &str) -> i32 {
    let game_scheme = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 1), ("Z", 2)])),
        ("B", HashMap::from([("X", 1), ("Y", 2), ("Z", 3)])),
        ("C", HashMap::from([("X", 2), ("Y", 3), ("Z", 1)])),
    ]);

    let score = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        let uline = line.unwrap();
        let splitted: Vec<&str> = uline.trim().split(" ").collect();

        let (player1, player2) = (splitted[0], splitted[1]);
        let points = game_scheme[player1][player2] + score[player2];

        total_score += points;
    }

    return total_score;
}
