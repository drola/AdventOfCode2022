/// Solution to an Advent of Code problem, day 16, 2022
/// https://adventofcode.com/2022/day/16

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();
}
