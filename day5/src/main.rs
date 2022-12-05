use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Please specify a file to read");

    let filepath: &str = &args[1];

    let sol1 = part_one(filepath);
    println!("Challenge 1: {}", sol1);

    let sol2 = part_two(filepath);
    println!("Challenge 2: {}", sol2);
}

fn part_one(filepath: &str) -> String {
    let file = File::open(filepath).expect("Unable to open file");
    let mut binding = BufReader::new(file);
    let reader = binding.by_ref();

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let chars = line.chars().chunks(4);
        let crates = chars.into_iter()
            .map(|c| c.collect_vec()[1])
            .filter(|c| !c.is_ascii_digit());

        for (i, c) in crates.enumerate() {
            if lines.len() <= i {
                if c == ' ' {
                    lines.push(vec![]);
                } else {
                    lines.push(vec![c]);
                }
            } else if c != ' ' {
                lines[i].push(c);
            }
        }
    }

    for line in reader.lines() {
        let line = line.unwrap();

        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(&line).unwrap();

        let times = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        for _ in 0..times {
            let stack = &lines[from];
            let crate_a = stack[0];

            lines[from].remove(0);
            lines[to].insert(0, crate_a);
        }
    }

    lines.into_iter().map(|l| l[0]).join("")
}


fn part_two(filepath: &str) -> String {
    let file = File::open(filepath).expect("Unable to open file");
    let mut binding = BufReader::new(file);
    let reader = binding.by_ref();

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let chars = line.chars().chunks(4);
        let crates = chars.into_iter()
            .map(|c| c.collect_vec()[1])
            .filter(|c| !c.is_ascii_digit());

        for (i, c) in crates.enumerate() {
            if lines.len() <= i {
                if c == ' ' {
                    lines.push(vec![]);
                } else {
                    lines.push(vec![c]);
                }
            } else if c != ' ' {
                lines[i].push(c);
            }
        }
    }

    for line in reader.lines() {
        let line = line.unwrap();

        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(&line).unwrap();

        let times = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;


        let stack = &lines[from];
        let mut crate_a = stack[..times].to_vec();
        crate_a.append(lines[to].as_mut());

        lines[from].drain(0..times);
        lines[to] = crate_a;
    }

    lines.into_iter().map(|l| l[0]).join("")
}
