/// Solution to an Advent of Code problem, day 02, 2022
/// https://adventofcode.com/2022/day/02

use std::env;
use std::fs;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut combinations_part_1 =  HashMap::new();
    combinations_part_1.insert("A X", 4); // 1 + 3 
    combinations_part_1.insert("A Y", 8); // 2 + 6 
    combinations_part_1.insert("A Z", 3); // 3 + 0
    combinations_part_1.insert("B X", 1); // 1 + 0
    combinations_part_1.insert("B Y", 5); // 2 + 3
    combinations_part_1.insert("B Z", 9); // 3 + 6
    combinations_part_1.insert("C X", 7); // 1 + 6
    combinations_part_1.insert("C Y", 2); // 2 + 0
    combinations_part_1.insert("C Z", 6); // 3 + 3 
    let score_part_1: i32= lines.clone().map(|l| combinations_part_1.get(l).unwrap()).sum();
    println!("Score (part 1): {}", score_part_1);
    
    let mut combinations_part_2 =  HashMap::new();
    combinations_part_2.insert("A X", 3); // lose; rock + scissors; 3 + 0
    combinations_part_2.insert("A Y", 4); // draw; rock + rock; 1 + 3
    combinations_part_2.insert("A Z", 8); //  win; rock + paper; 2 + 6
    combinations_part_2.insert("B X", 1); // lose; paper + rock; 1 + 0
    combinations_part_2.insert("B Y", 5); // draw; paper + paper; 2 + 3
    combinations_part_2.insert("B Z", 9); //  win; paper + scissors; 3 + 6
    combinations_part_2.insert("C X", 2); // lose; scissors + paper; 2 + 0
    combinations_part_2.insert("C Y", 6); // draw; scissors + scissors; 3 + 3
    combinations_part_2.insert("C Z", 7); //  win; scissors + rock; 1 + 6
    let score_part_2: i32= lines.map(|l| combinations_part_2.get(l).unwrap()).sum();
    println!("Score (part 2): {}", score_part_2);
}
