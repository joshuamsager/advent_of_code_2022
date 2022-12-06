use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;


fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Please specify a file to read");

    let filepath: &str = &args[1];

    let timer = Instant::now();
    let sol1 = part_one(filepath);
    let elapsed = timer.elapsed();
    println!("Challenge 1: {}, elapsed: {} ms", sol1, elapsed.as_millis());

    let timer = Instant::now();
    let sol2 = part_two(filepath);
    let elapsed = timer.elapsed();
    println!("Challenge 2: {}, elapsed: {} ms", sol2, elapsed.as_millis());
}

fn part_one(filepath: &str) -> usize {

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let chars = line.trim().chars().collect::<Vec<char>>();
        let mut i = 0;
        for window in chars.windows(4) {
            let set: HashSet<&char> = HashSet::from_iter(window);
            if set.len() == 4 {
                return i+4;
            }
            i+=1;
        }
    }

    0
}


fn part_two(filepath: &str) -> usize {

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let chars = line.trim().chars().collect::<Vec<char>>();
        let mut i = 0;
        for window in chars.windows(14) {
            let set: HashSet<&char> = HashSet::from_iter(window);
            if set.len() == 14 {
                return i+14;
            }
            i+=1;
        }
    }

    0
}
