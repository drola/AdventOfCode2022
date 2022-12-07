/// Solution to an Advent of Code problem, day 04, 2022
/// https://adventofcode.com/2022/day/04
use std::env;
use std::fs;

use nom::character::complete::{char, u64};
use nom::sequence::tuple;
use nom::IResult;

fn is_fully_contained(l: &Interval, r: &Interval) -> bool {
    return (l.a <= r.a && l.b >= r.b) || (r.a <= l.a && r.b >= l.b);
}

fn is_overlapping(l: &Interval, r: &Interval) -> bool {
    return (l.a <= r.a && l.b >= r.a) || (r.a <= l.a && r.b >= l.a);
}

#[derive(Debug, PartialEq)]
struct Interval {
    a: u64,
    b: u64,
}

fn interval(input: &str) -> IResult<&str, Interval> {
    let (input, (a, _, b)) = tuple((u64, char('-'), u64))(input)?;
    Ok((input, Interval { a, b }))
}

fn intervals(input: &str) -> IResult<&str, (Interval, Interval)> {
    let (input, (l, _, r)) = tuple((interval, char(','), interval))(input)?;
    Ok((input, (l, r)))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();
    let parsed_lines = lines
        .map(|l| intervals(l).unwrap().1)
        .collect::<Vec<(Interval, Interval)>>();

    let count_of_fully_contained = parsed_lines
        .iter()
        .map(|(l, r)| is_fully_contained(l, r))
        .filter(|&a| a)
        .count();
    println!(
        "[part 1] Count of fully contained: {}",
        count_of_fully_contained
    );
    let count_of_overlapping = parsed_lines
        .iter()
        .map(|(l, r)| is_overlapping(l, r))
        .filter(|&a| a)
        .count();
    println!("[part 2] Count of overlapping: {}", count_of_overlapping);
}
