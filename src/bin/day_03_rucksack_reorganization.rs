/// Solution to an Advent of Code problem, day 03, 2022
/// https://adventofcode.com/2022/day/03
use std::env;
use std::fs;

use itertools::Itertools;

fn priority(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        return 1 + u64::from(c) - u64::from('a');
    } else if c.is_ascii_uppercase() {
        return 27 + u64::from(c) - u64::from('A');
    } else {
        panic!("Invalid char: {}", c);
    }
}

fn find_common_item_priority_in_both_compartments(s: &str) -> u64 {
    let mut types_in_first_compartment = [false; 52 + 1];
    let chars = s.chars().collect::<Vec<char>>();
    let len = chars.len();
    let (first_compartment, second_compartment) = chars.split_at(len / 2);

    for &char in first_compartment {
        types_in_first_compartment[priority(char) as usize] = true;
    }
    for &char in second_compartment {
        if types_in_first_compartment[priority(char) as usize] {
            return priority(char);
        }
    }
    return 0;
}

fn find_badge_priority<'a, T: Iterator<Item = &'a str>>(s: T) -> u64 {
    let mut counts_by_priority = [0 as u8; 53];
    for line in s {
        let mut counted = [false; 53];
        for c in line.chars() {
            let p = priority(c) as usize;
            if !counted[p] {
                counted[p] = true;
                counts_by_priority[p] = counts_by_priority[p] + 1;
                if counts_by_priority[p] >= 3 {
                    return priority(c);
                }
            }
        }
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let s1: u64 = lines
        .clone()
        .map(find_common_item_priority_in_both_compartments)
        .sum();
    println!("[part 1] Priorities sum: {}", s1);

    let s2: u64 = lines
        .chunks(3)
        .into_iter()
        //.map(|c| c.by_ref())
        .map(find_badge_priority)
        .sum();
    println!("[part 2] Badge priorities sum: {}", s2);
}
