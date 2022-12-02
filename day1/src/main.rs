use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Please specify a file to read");

    let filepath: &str = &args[1];

    let max_calories: i32 = part_one(filepath);
    println!("Max calories: {}", max_calories);

    let top_three_calories: i32 = part_two(filepath, 3);
    println!("Max top three calories: {}", top_three_calories);
}

fn part_one(filepath: &str) -> i32 {
    let mut max_calories: i32 = 0;
    let mut running_total: i32 = 0;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let raw_line = line.unwrap();
        let calories = raw_line.trim();
        if calories.is_empty() {
            if running_total > max_calories {
                max_calories = running_total;
            }
            running_total = 0;
        } else {
            let parsed_calories = calories.parse::<i32>().unwrap();
            running_total += parsed_calories;
        }
    }

    return max_calories;
}

fn part_two(filepath: &str, n: usize) -> i32 {
    let mut running_calories = vec![0; n];
    let mut min_calories = 0;
    let mut running_total: i32 = 0;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let raw_line = line.unwrap();
        let calories = raw_line.trim();
        if calories.is_empty() {
            if running_total > min_calories {
                let pos = running_calories.partition_point(|&x| x < running_total);
                running_calories.insert(pos, running_total);

                running_calories.remove(0);
                min_calories = running_calories[0];
            }
            running_total = 0;
        } else {
            let parsed_calories = calories.parse::<i32>().unwrap();
            running_total += parsed_calories;
        }
    }

    return running_calories.iter().sum();
}
