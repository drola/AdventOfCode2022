use std::cmp::max;
/// Solution to an Advent of Code problem, day 08, 2022
/// https://adventofcode.com/2022/day/08
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let map: Vec<Vec<i64>> = lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    let w = map[0].len();
    let h = map.len();

    let mut is_visible = vec![vec![false; w]; h];

    // Left -> Right scan, row-wise
    for y in 0..h {
        let mut max_h = -1;
        for x in 0..w {
            is_visible[y][x] = is_visible[y][x] || map[y][x] > max_h;
            max_h = max(max_h, map[y][x]);
        }
    }

    // Right -> Left scan, row-wise
    for y in 0..h {
        let mut max_h = -1;
        for x in (0..w).rev() {
            is_visible[y][x] = is_visible[y][x] || map[y][x] > max_h;
            max_h = max(max_h, map[y][x]);
        }
    }

    // Top -> Down scan, column-wise
    for x in 0..w {
        let mut max_h = -1;
        for y in 0..h {
            is_visible[y][x] = is_visible[y][x] || map[y][x] > max_h;
            max_h = max(max_h, map[y][x]);
        }
    }

    // Down -> Up scan, column-wise
    for x in 0..w {
        let mut max_h = -1;
        for y in (0..h).rev() {
            is_visible[y][x] = is_visible[y][x] || map[y][x] > max_h;
            max_h = max(max_h, map[y][x]);
        }
    }

    let visible_count: u64 = is_visible
        .iter()
        .map(|s| s.iter().filter(|&&v| v).count() as u64)
        .sum();
    println!("Visible count: {}", visible_count);

    let mut scenic_scores = vec![vec![1 as u64; w]; h];

    for x in 1..w - 1 {
        for y in 1..h - 1 {
            let mut c = 0;

            // Up
            for j in (0..y).rev() {
                c = c + 1;
                if map[j][x] >= map[y][x] {
                    break;
                }
            }

            scenic_scores[y][x] = scenic_scores[y][x] * c;

            // Down
            c = 0;
            for j in (y + 1)..h {
                c = c + 1;
                if map[j][x] >= map[y][x] {
                    break;
                }
            }
            scenic_scores[y][x] = scenic_scores[y][x] * c;

            // Left
            c = 0;
            for i in (0..x).rev() {
                c = c + 1;
                if map[y][i] >= map[y][x] {
                    break;
                }
            }
            scenic_scores[y][x] = scenic_scores[y][x] * c;

            // Right
            c = 0;
            for i in (x + 1)..w {
                c = c + 1;
                if map[y][i] >= map[y][x] {
                    break;
                }
            }
            scenic_scores[y][x] = scenic_scores[y][x] * c;
        }
    }

    let max_scenic_score = scenic_scores
        .iter()
        .map(|s| s.iter().max().unwrap())
        .max()
        .unwrap();
    println!("Max scenic score [part 2]: {}", max_scenic_score);
}
