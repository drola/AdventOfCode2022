/// Solution to an Advent of Code problem, day 01, 2022
/// https://adventofcode.com/2022/day/01
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut max_cals: i64 = 0;
    let mut current_cals: i64 = 0;
    let mut sums: Vec<i64> = vec![];

    for line in lines {
        let cals: Result<i64, _> = line.parse();
        if cals.is_ok() {
            current_cals = current_cals + cals.unwrap();
        } else {
            sums.push(current_cals);
            if current_cals > max_cals {
                max_cals = current_cals;
            }
            current_cals = 0;
        }
    }
    if current_cals > max_cals {
        max_cals = current_cals;
    }
    sums.push(current_cals);
    println!("Max cals: {}", max_cals);

    sums.sort();
    sums.reverse();

    println!("Top 3 sum: {} + {} + {} = {}", sums[0], sums[1], sums[2], sums[0]+sums[1]+sums[2]);
}
