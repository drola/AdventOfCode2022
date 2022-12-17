use std::cmp::max;
use std::cmp::min;
/// Solution to an Advent of Code problem, day 17, 2022
/// https://adventofcode.com/2022/day/17
use std::env;
use std::fs;
use std::panic::UnwindSafe;
use std::str;
use std::time::Instant;

const ROCK: u8 = "#".as_bytes()[0];
const EMPTY: u8 = ".".as_bytes()[0];
const LEFT: u8 = "<".as_bytes()[0];
const RIGHT: u8 = ">".as_bytes()[0];

fn is_legal_position(
    tower: &Vec<[u8; 7]>,
    rock_pattern: &[[u8; 4]; 4],
    bottom: i64,
    left: i64,
) -> bool {
    if bottom as usize >= tower.len() {
        //println!("bottom ({}) >= tower.len() ({})", bottom, tower.len());
        return false;
    }

    for y in 0..(4 as i64) {
        for x in 0..(4 as i64) {
            if rock_pattern[y as usize][x as usize] == ROCK
                && ((left + x) >= 7
                    || (left + x) < 0
                    || tower[(bottom - 3 + y) as usize][(left + x) as usize] == ROCK)
            {
                //println!("left+x = {}, bottom-3+y={}", left+x, bottom-3+y);
                return false;
            }
        }
    }
    return true;
}

fn settle(tower: &mut Vec<[u8; 7]>, rock_pattern: &[[u8; 4]; 4], bottom: i64, left: i64) -> usize {
    let mut highest_rock = bottom as usize;
    for y in 0..(4 as i64) {
        for x in 0..(4 as i64) {
            if rock_pattern[y as usize][x as usize] == ROCK {
                tower[(bottom - 3 + y) as usize][(left + x) as usize] = ROCK;
                highest_rock = min(highest_rock, (bottom - 3 + y) as usize);
            }
        }
    }
    return highest_rock;
}

fn draw(tower: &Vec<[u8; 7]>) {
    for y in (0..20).rev() {
        println!("|{}|", str::from_utf8(&tower[tower.len() - y - 1]).unwrap());
    }

    println!("+-------+");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let jets = contents.lines().next().unwrap().as_bytes();

    let rock_pattern_0 = [
        [EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY, EMPTY],
        [ROCK, ROCK, ROCK, ROCK],
    ];
    let rock_pattern_1 = [
        [EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, ROCK, EMPTY, EMPTY],
        [ROCK, ROCK, ROCK, EMPTY],
        [EMPTY, ROCK, EMPTY, EMPTY],
    ];
    let rock_pattern_2 = [
        [EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, ROCK, EMPTY],
        [EMPTY, EMPTY, ROCK, EMPTY],
        [ROCK, ROCK, ROCK, EMPTY],
    ];
    let rock_pattern_3 = [
        [ROCK, EMPTY, EMPTY, EMPTY],
        [ROCK, EMPTY, EMPTY, EMPTY],
        [ROCK, EMPTY, EMPTY, EMPTY],
        [ROCK, EMPTY, EMPTY, EMPTY],
    ];
    let rock_pattern_4 = [
        [EMPTY, EMPTY, EMPTY, EMPTY],
        [EMPTY, EMPTY, EMPTY, EMPTY],
        [ROCK, ROCK, EMPTY, EMPTY],
        [ROCK, ROCK, EMPTY, EMPTY],
    ];

    let rock_patterns = [
        rock_pattern_0,
        rock_pattern_1,
        rock_pattern_2,
        rock_pattern_3,
        rock_pattern_4,
    ];

    let mut tower = vec![[EMPTY; 7]; 2022 * 4 + 10];
    let mut highest_rock = tower.len(); // current highest rock (or floor)
    let mut jet_index: usize = 0;
    let t_start = Instant::now();
    let mut max_heights_spread = 0;
    for rock_i in 0..2022 {
        //println!("Rock {}", rock_i);

        let mut left: i64 = 2;
        let mut bottom: i64 = highest_rock as i64 - 4;
        let rock_pattern = &rock_patterns[rock_i % rock_patterns.len()];

        loop {
            /*match jets[jet_index] {
                LEFT => println!("Jet left!"),
                _ => println!("Jet right!"),
            }*/
            let left_after_jet = match jets[jet_index] {
                LEFT => left - 1,
                _ => left + 1,
            };
            if is_legal_position(&tower, rock_pattern, bottom, left_after_jet) {
                //println!("Jet applied; left = {}", left_after_jet);
                left = left_after_jet;
            } else {
                //println!("({},{}) would not be a legal position.", bottom, left_after_jet);
            }

            jet_index = (jet_index + 1) % jets.len();

            if !is_legal_position(&tower, rock_pattern, bottom + 1, left) {
                highest_rock = min(highest_rock, settle(&mut tower, rock_pattern, bottom, left));
                /*println!(
                    "Settle at {}, {}; highest rock = {}",
                    bottom, left, highest_rock
                );*/
                break;
            }
            bottom = bottom + 1;
        }

        //draw(&tower);

        /*let mut max_heights = [0 as usize; 7];
        for (y, row) in tower.iter().enumerate() {
            for i in 0..7 {
                if row[i] == ROCK {
                    max_heights[i] = max(max_heights[i], tower.len() - y);
                }
            }
        }
        println!("{:?}", max_heights);
        max_heights_spread = max(
            max_heights_spread,
            max_heights.iter().max().unwrap() - max_heights.iter().min().unwrap(),
        );*/
    }
    println!("{:?}", Instant::now().duration_since(t_start));
    println!(
        "Tower height [part 1]: {} ({}-{})",
        tower.len() - highest_rock,
        tower.len(),
        highest_rock
    );

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



    // soooo... ??
    // perhaps the pattern repeats?
    // jet pattern: 10091 chars
    // rock patterns: 5
    // "floor" has different pattern each time...
    // floor pattern is up to 50 cells high;
    // therefore, the floor has 2^(50*7) states.
    // kilo (10^3), mega (10^6), giga (10^9)
}
