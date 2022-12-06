/// Solution to an Advent of Code problem, day 06, 2022
/// https://adventofcode.com/2022/day/06

use std::env;
use std::fs;

fn is_start_of_packet(c: &[u8]) -> bool {
    if c.len() < 4 {
        return false;
    }

    for i in 0..4 {
        for j in (i+1)..4 {
            if c[i] == c[j] {
                return false;
            }
        }
    }

    return true;
}

fn is_start_of_message(c: &[u8]) -> bool {
    if c.len() < 14 {
        return false;
    }

    // This is crude. But it still works well at 14 characters.
    for i in 0..14 {
        for j in (i+1)..14 {
            if c[i] == c[j] {
                return false;
            }
        }
    }

    return true;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    for i in 4..contents.len() {
        if is_start_of_packet(&contents.as_bytes()[i-4..i]) {
            println!("[part 1]: {}", i);
            break;
        }
    }

    for i in 14..contents.len() {
        if is_start_of_message(&contents.as_bytes()[i-14..i]) {
            println!("[part 2]: {}", i);
            break;
        }
    }

}
