/// Solution to an Advent of Code problem, day 20, 2022
/// https://adventofcode.com/2022/day/20
use std::env;
use std::fs;

use itertools::Itertools;

fn remix(numbers: &mut Vec<i64>, new_positions: &mut Vec<usize>) {
    let count = numbers.len();
    let mut _i: usize = 0;
    loop {
        if _i >= count {
            break;
        }
        let i = new_positions.iter().find_position(|&&p| p == _i).unwrap().0;
        _i = _i + 1;

        let number = numbers[i];

        let mut new_index = i as i64;
        if number > 0 {
            new_index = new_index + number;
            let overflows_count = new_index / (count as i64 - 1);
            new_index = new_index - (count as i64 - 1) * overflows_count;
            if new_index >= count as i64 {
                new_index = new_index - count as i64 + 1;
            }
        }
        if number < 0 {
            new_index = new_index + number;
            let overflows_count = new_index.abs() / (count as i64 - 1);
            new_index = new_index + overflows_count * (count as i64 - 1);
            if new_index <= 0 {
                new_index = new_index + count as i64 - 1;
            }
        }
        numbers.remove(i);
        let np = new_positions.remove(i);
        numbers.insert(new_index as usize, number);
        new_positions.insert(new_index as usize, np);
    }
}

fn the_sum(numbers: &Vec<i64>) -> i64 {
    let count = numbers.len();
    let index_of_0 = numbers.iter().find_position(|&&n| n == 0).unwrap().0;
    numbers[(index_of_0 + 1000) % count]
        + numbers[(index_of_0 + 2000) % count]
        + numbers[(index_of_0 + 3000) % count]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let numbers = contents
        .lines()
        .map(|f| f.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let positions = (0..numbers.len()).collect::<Vec<usize>>();

    // Part 1
    let mut numbers_p1 = numbers.clone();
    let mut positions_p1 = positions.clone();
    remix(&mut numbers_p1, &mut positions_p1);
    println!("The sum [part 1]: {}", the_sum(&numbers_p1));

    // Part 2
    let mut numbers_p2 = numbers.iter().map(|&n| n * 811589153).collect_vec();
    let mut positions_p2 = positions.clone();
    for _ in 0..10 {
        remix(&mut numbers_p2, &mut positions_p2);
        // println!("{:?}", numbers_p2.as_slice().get(0..7).unwrap());
    }
    println!("The sum [part 2]: {}", the_sum(&numbers_p2));
}
