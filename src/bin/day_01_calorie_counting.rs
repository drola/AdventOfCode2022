/// Solution to an Advent of Code problem, day 01, 2022
/// https://adventofcode.com/2022/day/01
use itertools::Itertools;
use std::cmp::Reverse;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines().map(|line| line.parse::<u64>());
    let top_3_sums = lines
        .group_by(|r| r.is_ok())
        .into_iter()
        .filter(|(k, _g)| *k)
        .map(|(_k, g)| Reverse(g.map(|n| n.unwrap()).sum::<u64>()))
        .k_smallest(3)
        .map(|n| n.0) // Remove Reverse()
        .collect::<Vec<_>>();
    println!("Top 3 sums: {:?}", top_3_sums);

    println!(
        "Elf with the most calories is carrying {} calories.",
        top_3_sums[0]
    );
    println!(
        "Top three elves are carrying a total of {} calories",
        top_3_sums.iter().sum::<u64>()
    );
}
