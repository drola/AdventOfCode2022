use std::cmp::max;
use std::cmp::min;
/// Solution to an Advent of Code problem, day 14, 2022
/// https://adventofcode.com/2022/day/14
use std::env;
use std::fs;
use std::str;

const GRAIN: u8 = "o".as_bytes()[0];
const ROCK: u8 = "#".as_bytes()[0];
const EMPTY: u8 = ".".as_bytes()[0];

#[allow(dead_code)]
fn draw(grid: &Vec<Vec<u8>>, x0: usize, x1: usize, y0: usize, y1: usize) {
    for y in y0..y1 {
        println!("{}", str::from_utf8(&grid[y][x0..x1]).unwrap())
    }
}

fn pour_the_sand(grid: &mut Vec<Vec<u8>>) -> u64 {
    let mut grains_that_came_to_rest: u64 = 0;
    loop {
        let mut x = 500;
        let mut at_rest = false;

        if grid[0][x] != EMPTY {
            break;
        }

        for y in 1..grid.len() {
            if grid[y][x] == EMPTY {
                // stay
            } else if grid[y][x - 1] == EMPTY {
                x = x - 1;
            } else if grid[y][x + 1] == EMPTY {
                x = x + 1;
            } else {
                at_rest = true;
                grid[y - 1][x] = GRAIN;
                break;
            }
        }
        if at_rest {
            grains_that_came_to_rest = grains_that_came_to_rest + 1;
        } else {
            break;
        }
    }

    return grains_that_came_to_rest;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let max_x = 1024;
    let max_y = 1024;

    let mut grid = vec![vec![EMPTY; max_x]; max_y];

    let mut floor_y: usize = 0;

    // Parse and fill grid
    for line in lines {
        let mut prev_xy: Option<(usize, usize)> = None;

        for xystr in line.split(" -> ") {
            let coords: Vec<usize> = xystr
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let (x, y) = (coords[0], coords[1]);
            grid[y][x] = ROCK;

            // Fill the line
            match prev_xy {
                Some((prev_x, prev_y)) if prev_x == x => {
                    for y in min(prev_y, y)..max(prev_y, y) + 1 {
                        grid[y][x] = ROCK;
                    }
                }
                Some((prev_x, prev_y)) if prev_y == y => {
                    for x in min(prev_x, x)..max(prev_x, x) + 1 {
                        grid[y][x] = ROCK;
                    }
                }
                _ => {}
            };

            prev_xy = Some((x, y));

            floor_y = max(y + 2, floor_y);
        }
    }

    let mut grid_p2 = grid.clone();
    for x in 0..max_x {
        grid_p2[floor_y][x] = ROCK;
    }



    println!("Grains at rest [part 1]: {}", pour_the_sand(&mut grid));
    println!("Grains at rest [part 2]: {}", pour_the_sand(&mut grid_p2));
}
