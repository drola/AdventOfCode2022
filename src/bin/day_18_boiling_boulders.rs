use std::collections::VecDeque;
/// Solution to an Advent of Code problem, day 18, 2022
/// https://adventofcode.com/2022/day/18
use std::env;
use std::fs;

use itertools::Itertools;

fn count_foggy_faces(shape: &[[[bool; 32]; 32]; 32], is_foggy: &[[[bool; 32]; 32]; 32]) -> u64 {
    let mut count = 0;

    // Find exposed faces in along X axis
    for z in 0..32 {
        for y in 0..32 {
            for x in 1..31 {
                if shape[z][y][x] && is_foggy[z][y][x - 1] {
                    count = count + 1;
                }
                if shape[z][y][x] && is_foggy[z][y][x + 1] {
                    count = count + 1;
                }
            }
        }
    }
    // Find exposed faces in along Y axis
    for z in 0..32 {
        for x in 0..32 {
            for y in 1..31 {
                if shape[z][y][x] && is_foggy[z][y - 1][x] {
                    count = count + 1;
                }
                if shape[z][y][x] && is_foggy[z][y + 1][x] {
                    count = count + 1;
                }
            }
        }
    }
    // Find exposed faces in along Z axis
    for x in 0..32 {
        for y in 0..32 {
            for z in 1..31 {
                if shape[z][y][x] && is_foggy[z - 1][y][x] {
                    count = count + 1;
                }
                if shape[z][y][x] && is_foggy[z + 1][y][x] {
                    count = count + 1;
                }
            }
        }
    }
    return count;
}

fn invert_3d_slice(slice: &[[[bool;32];32];32]) -> [[[bool; 32]; 32]; 32] {
    let mut inverted = slice.clone();
    for i in 0..slice.len() {
        for j in 0..slice[i].len() {
            for k in 0..slice[i][j].len() {
                inverted[i][j][k] = !inverted[i][j][k];
            }
        }
    }
    return inverted;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut shape = [[[false; 32]; 32]; 32];

    for line in lines {
        let (x, y, z) = line
            .split(",")
            .map(|n| n.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();

        shape[z as usize + 1][y as usize + 1][x as usize + 1] = true;
    }

    let count_part_1: u64 = count_foggy_faces(&shape, &invert_3d_slice(&shape));
    println!("Exposed faces [part 1]: {}", count_part_1);

    // For part 2, mark parts, that cannot be reached by fog
    let mut is_foggy = [[[false; 32]; 32]; 32];
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();

    for i in 0..32 {
        for j in 0..32 {
            queue.push_back((0, i, j)); // plane x=0
            queue.push_back((31, i, j)); // plane x=31
            queue.push_back((i, 0, j)); // plane y=0
            queue.push_back((i, 31, j)); // plane y=31
            queue.push_back((i, j, 0)); // plane z=0
            queue.push_back((i, j, 31)); // plane z=31
        }
    }
    while let Some((x, y, z)) = queue.pop_front() {
        is_foggy[x][y][z] = true;

        if x > 0 && !shape[x - 1][y][z] && !is_foggy[x - 1][y][z] {
            queue.push_back((x - 1, y, z));
        }
        if x < 31 && !shape[x + 1][y][z] && !is_foggy[x + 1][y][z] {
            queue.push_back((x + 1, y, z));
        }
        if y > 0 && !shape[x][y - 1][z] && !is_foggy[x][y - 1][z] {
            queue.push_back((x, y - 1, z));
        }
        if y < 31 && !shape[x][y + 1][z] && !is_foggy[x][y + 1][z] {
            queue.push_back((x, y + 1, z));
        }
        if z > 0 && !shape[x][y][z - 1] && !is_foggy[x][y][z - 1] {
            queue.push_back((x, y, z - 1));
        }
        if z < 31 && !shape[x][y][z + 1] && !is_foggy[x][y][z + 1] {
            queue.push_back((x, y, z + 1));
        }
    }

    let count_part_2: u64 = count_foggy_faces(&shape, &is_foggy);
    println!("Exposed faces [part 2]: {}", count_part_2);
}
