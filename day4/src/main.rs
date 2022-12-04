use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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
    let mut count = 0;
    let file = File::open(filepath).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(",");
        let (a, b) = (split.next().unwrap(), split.next().unwrap());

        split = a.split("-");
        let ( ax,  bx) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

        split = b.split("-");
        let (ay, by) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

        if ax >= ay && bx <= by {
            count += 1;
        } else if bx >= by && ax <= ay {
            count += 1;
        }
    }
    return count
}


fn part_two(filepath: &str) -> i32 {
    let mut count = 0;
    let file = File::open(filepath).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(",");
        let (a, b) = (split.next().unwrap(), split.next().unwrap());

        split = a.split("-");
        let ( ax,  bx) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

        split = b.split("-");
        let (ay, by) = (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap());


        if ax <= by && bx >= ay {
            count+=1;
        }
    }
    return count
}
