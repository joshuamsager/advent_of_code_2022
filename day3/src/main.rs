use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Please specify a file to read");

    let filepath: &str = &args[1];

    let sol1: i32 = part_one(filepath);
    println!("Challenge 1: {}", sol1);

    let sol2: i32 = part_two(filepath);
    println!("Challenge 2: {}", sol2);
}


fn part_one(filepath: &str) -> i32 {
    let mut total_sum = 0;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    for line in reader.lines() {
        let line = line.unwrap();

        let (a, b) = line.split_at((line.chars().count()) / 2);

        let set_a = HashSet::<char>::from_iter(a.chars());
        let set_b = HashSet::<char>::from_iter(b.chars());

        let mut intersection = set_a.intersection(&set_b);

        let m = intersection.next().unwrap();


        let pos = i32::try_from(alphabet.iter().position(|&r| r == *m).unwrap()).unwrap();

        total_sum += pos + 1;
    }
    return total_sum;
}


fn part_two(filepath: &str) -> i32 {
    let mut total_sum = 0;

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    for lines in &reader.lines().chunks(3) {
        let mut iter = lines.enumerate();
        let first = iter.next();

        let (_, fline) = first.unwrap();
        let mut base = HashSet::<char>::from_iter(fline.unwrap().chars());

        for (_, line) in iter {
            let line = line.unwrap();
            let set = HashSet::<char>::from_iter(line.chars());

            base = base.intersection(&set).cloned().collect();
        }

        let m = base.iter().next().unwrap();

        let pos = i32::try_from(alphabet.iter().position(|&r| r == *m).unwrap()).unwrap();
        total_sum += pos + 1;
    }
    return total_sum;
}
