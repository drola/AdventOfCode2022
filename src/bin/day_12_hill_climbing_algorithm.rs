/// Solution to an Advent of Code problem, day 12, 2022
/// https://adventofcode.com/2022/day/12
use std::cmp::min;
use std::collections::VecDeque;
use std::env;
use std::fs;


fn schedule_valid_moves(
    queue: &mut VecDeque<(usize, usize, usize, usize)>,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) {
    if x > 0 {
        queue.push_back((x, y, x - 1, y));
    }
    if x + 1 < w {
        queue.push_back((x, y, x + 1, y));
    }
    if y > 0 {
        queue.push_back((x, y, x, y - 1));
    }
    if y + 1 < h {
        queue.push_back((x, y, x, y + 1));
    }
}

fn find_shortest_path(
    map: &Vec<Vec<u8>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> u64 {
    let w = map[0].len();
    let h = map.len();
    let mut required_steps = vec![vec![u64::MAX; w]; h];
    required_steps[start_y][start_x] = 0;

    let mut next_positions: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    schedule_valid_moves(&mut next_positions, start_x, start_y, w, h);

    while let Some(position) = next_positions.pop_front() {
        let (from_x, from_y, x, y) = position;
        if required_steps[from_y][from_x] + 1 < required_steps[y][x]
            && (map[y][x] <= map[from_y][from_x] + 1)
        {
            required_steps[y][x] = required_steps[from_y][from_x] + 1;
            schedule_valid_moves(&mut next_positions, x, y, w, h);
        }
    }

    required_steps[end_y][end_x]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut map = lines
        .filter(|l| l.len() > 0)
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let w = map[0].len();
    let h = map.len();

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    let mut end_x: usize = 0;
    let mut end_y: usize = 0;

    // Scan for start and end coordinates
    for y in 0..h {
        for x in 0..w {
            if map[y][x] == "S".as_bytes()[0] {
                start_x = x;
                start_y = y;
            }
            if map[y][x] == "E".as_bytes()[0] {
                end_x = x;
                end_y = y;
            }
        }
    }

    map[start_y][start_x] = "a".as_bytes()[0];
    map[end_y][end_x] = "z".as_bytes()[0];

    println!(
        "Steps [part 1]: {}",
        find_shortest_path(&map, start_x, start_y, end_x, end_y)
    );

    let mut shortest_path_multiple_starts = u64::MAX;
    for x in 0..w {
        for y in 0..h {
            if map[y][x] == "a".as_bytes()[0] {
                shortest_path_multiple_starts = min(
                    shortest_path_multiple_starts,
                    find_shortest_path(&map, x, y, end_x, end_y),
                );
            }
        }
    }
    println!("Steps [part 2]: {}", shortest_path_multiple_starts);
}
