/// Solution to an Advent of Code problem, day 17, 2022
/// https://adventofcode.com/2022/day/17
use std::env;
use std::fs;
use std::time::Instant;

const LEFT: u8 = "<".as_bytes()[0];
const N: usize = 128;

fn is_legal_position_u8(tower: &[u8; N], rock_pattern: [u8; 4], top: i64, left: i64) -> bool {
    if top < 0 || left < 0 {
        return false;
    }

    for y in 0..(4 as i64) {
        let line = rock_pattern[y as usize] >> left;
        if line & 0b00000001 > 0 {
            return false;
        }
        if line & tower[(top + y) as usize % N] > 0 {
            return false;
        }
    }
    return true;
}

fn settle_u8(tower: &mut [u8; N], rock_pattern: [u8; 4], top: i64, left: i64) -> i64 {
    let mut highest_rock = top;
    for y in 0..(4 as i64) {
        let i = (top + y) as usize % N;
        tower[i] = tower[i] | (rock_pattern[y as usize] >> left);
        if tower[i] > 0 {
            highest_rock = top + y;
        }
    }
    return highest_rock;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let jets = contents
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|&b| match b {
            LEFT => -1,
            _ => 1,
        })
        .collect::<Vec<i64>>();
    let jets_len = jets.len();

    let mut rock_pattern_0_u8 = [0b00000000, 0b00000000, 0b00000000, 0b11110000];
    let mut rock_pattern_1_u8 = [0b00000000, 0b01000000, 0b11100000, 0b01000000];
    let mut rock_pattern_2_u8 = [0b00000000, 0b00100000, 0b00100000, 0b11100000];
    let mut rock_pattern_3_u8 = [0b10000000, 0b10000000, 0b10000000, 0b10000000];
    let mut rock_pattern_4_u8 = [0b00000000, 0b00000000, 0b11000000, 0b11000000];
    rock_pattern_0_u8.reverse();
    rock_pattern_1_u8.reverse();
    rock_pattern_2_u8.reverse();
    rock_pattern_3_u8.reverse();
    rock_pattern_4_u8.reverse();

    let rock_patterns_u8 = [
        rock_pattern_0_u8,
        rock_pattern_1_u8,
        rock_pattern_2_u8,
        rock_pattern_3_u8,
        rock_pattern_4_u8,
    ];

    //let mut tower = vec![0 as u8; 2022 * 4 + 10];
    let mut tower = [0 as u8; N];
    let mut highest_rock = -1;
    let mut jet_index: usize = 0;
    let t_start = Instant::now();
    for rock_i in 0..1000000000000 {
        let mut left: i64 = 2;
        let mut top: i64 = highest_rock + 4;
        let rock_pattern = rock_patterns_u8[rock_i % 5];
        /*tower[(highest_rock+1) as usize % N] = 0;
        tower[(highest_rock+2) as usize % N] = 0;
        tower[(highest_rock+3) as usize % N] = 0;
        tower[(highest_rock+4) as usize % N] = 0;*/
        tower[(highest_rock + 5) as usize % N] = 0;
        tower[(highest_rock + 6) as usize % N] = 0;
        tower[(highest_rock + 7) as usize % N] = 0;
        tower[(highest_rock + 8) as usize % N] = 0;

        loop {
            let left_after_jet = left + jets[jet_index];

            if is_legal_position_u8(&tower, rock_pattern, top, left_after_jet) {
                left = left_after_jet;
            }

            jet_index = (jet_index + 1) % jets_len;

            if !is_legal_position_u8(&tower, rock_pattern, top - 1, left) {
                let new_highest = settle_u8(&mut tower, rock_pattern, top, left);
                if new_highest > highest_rock {
                    highest_rock = new_highest;
                }
                //highest_rock = max(highest_rock, settle_u8(&mut tower, rock_pattern, top, left));
                break;
            }
            top = top - 1;
        }

        if rock_i == 2021 {
            println!("Tower height [part 1]: {}", highest_rock + 1);
        }
    }
    println!("{:?}", Instant::now().duration_since(t_start));
    println!("Tower height [part 2]: {}", highest_rock + 1);

    //println!("Max heights spread: {}", max_heights_spread);

    // TODO: Simulate 1000000000000 rocks. = 10^12
    // TODO: This means recycling tower memory. Possibly optimizing the algorithm.
    // theoretically, it might be possible to do 10^8 - 10^9 rocks/second.

    // Debug build:
    // 10ms for 2022 rocks
    // 1000ms for 2*10^5 rocks
    // 10^7s for 10^12 rocks
    // Prod build:
    // 2*10-4s for 2*10^3 rocks
    // 2*10-1s for 2*10^6 rocks
    // 2*10^2s for 2*10^9 rocks
    // 2*10^5s for 2*10^12 rocks = cca 3h?
    // TODO: Next step: replace [u8;7] with u8

    // u8 version:
    // 100us for 2022 rocks;
    // 10^-4s for 2*10^3 rocks
    // 10^-1s for 2*10^6 rocks
    // 10^2s for 2*10^9 rocks
    // 0.5 * 10^5s for 10^12 rocks = 13hr

    // soooo... ??
    // perhaps the pattern repeats?
    // jet pattern: 10091 chars
    // rock patterns: 5
    // "floor" has different pattern each time...
    // floor pattern is up to 50 cells high;
    // therefore, the floor has 2^(50*7) states.
    // kilo (10^3), mega (10^6), giga (10^9)

    // 31ms for 1e6 == 0.031s for 1e6
    // 31s for 1e9
    // 31.000s for 1e12 = ~10h
}
