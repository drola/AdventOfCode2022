/// Solution to an Advent of Code problem, day 04, 2022
/// https://adventofcode.com/2022/day/04
use std::env;
use std::fs;

fn is_fully_contained(a: u64, b: u64, x: u64, y: u64) -> bool {
    return (a <= x && b >= y) || (x <= a && y >= b);
}

fn is_overlapping(a: u64, b: u64, x: u64, y: u64) -> bool {
    return (a <= x && b >= x) || (x <= a && y >= a);
}

fn parse_line(s: &str) -> (u64, u64, u64, u64) {
    let mut numbers = s.split(&['-', ',']).map(|s| s.parse::<u64>().unwrap());
    return (
        numbers.next().unwrap(),
        numbers.next().unwrap(),
        numbers.next().unwrap(),
        numbers.next().unwrap(),
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();
    let parsed_lines = lines.map(parse_line);

    let count_of_fully_contained = parsed_lines.clone()
        .map(|(a, b, x, y)| is_fully_contained(a, b, x, y))
        .filter(|&a| a)
        .count();
    println!("[part 1] Count of fully contained: {}", count_of_fully_contained);
    
    let count_of_overlapping = parsed_lines
        .map(|(a, b, x, y)| is_overlapping(a, b, x, y))
        .filter(|&a| a)
        .count();
    println!("[part 2] Count of overlapping: {}", count_of_overlapping);
}
