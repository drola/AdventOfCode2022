/// Solution to an Advent of Code problem, day 10, 2022
/// https://adventofcode.com/2022/day/10
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut lines = contents.lines();

    let mut signal_strengths = 0;
    let mut x: i64 = 1;
    let mut pipeline: VecDeque<i64> = VecDeque::new();
    let mut cycle = 1;

    let mut picture = vec![vec![false; 40]; 6];

    loop {
        let line = lines.next();
        if cycle == 20
            || cycle == 60
            || cycle == 100
            || cycle == 140
            || cycle == 180
            || cycle == 220
        {
            signal_strengths = signal_strengths + cycle * x;
            println!("{} * {}", cycle, x);
        }
        println!("{}: x = {}", cycle, x);

        let py = (cycle - 1) / 40;
        let px = (cycle - 1) % 40;
        if x - 1 <= (px) && (px) <= x + 1 {
            picture[py as usize][px as usize] = true;
        }

        if line.is_some() {
            let l = line.unwrap();

            if l.starts_with("addx") {
                let mut i = l.split_ascii_whitespace();
                i.next();
                let factor: i64 = i.next().unwrap().parse().unwrap();
                pipeline.push_front(0);
                pipeline.push_front(factor);
            } else if l == "noop" {
                pipeline.push_front(0);
            }
        }
        if let Some(factor) = pipeline.pop_back() {
            x = x + factor;
        }
        cycle = cycle + 1;

        if !line.is_some() && pipeline.is_empty() && cycle > 220 {
            break;
        }
    }

    println!("Signal strengths: {}", signal_strengths);

    println!("");
    for y in 0..6 {
        for x in 0..40 {
            if picture[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
