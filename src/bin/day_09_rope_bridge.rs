/// Solution to an Advent of Code problem, day 09, 2022
/// https://adventofcode.com/2022/day/09
use std::collections::HashSet;
use std::env;
use std::fs;

fn next_following_knot_position(following: (i64, i64), leading: (i64, i64)) -> (i64, i64) {
    if (following.0 == leading.0 && following.1 == leading.1)
        || (following.0 == leading.0 && (following.1 == leading.1 + 1 || following.1 == leading.1 - 1))
        || (following.1 == leading.1 && (following.0 == leading.0 + 1 || following.0 == leading.0 - 1))
        || (following.0 == leading.0 - 1 && following.1 == leading.1 - 1)
        || (following.0 == leading.0 + 1 && following.1 == leading.1 + 1)
        || (following.0 == leading.0 + 1 && following.1 == leading.1 - 1)
        || (following.0 == leading.0 - 1 && following.1 == leading.1 + 1)
    {
        return following;
    }

    if following.0 == leading.0 && following.1 + 2 == leading.1 {
        return (following.0, following.1 + 1);
    }
    if following.0 == leading.0 && following.1 - 2 == leading.1 {
        return (following.0, following.1 - 1);
    }
    if following.1 == leading.1 && following.0 + 2 == leading.0 {
        return (following.0 + 1, following.1);
    }
    if following.1 == leading.1 && following.0 - 2 == leading.0 {
        return (following.0 - 1, following.1);
    }

    if leading.0 > following.0 && leading.1 > following.1 {
        return (following.0 + 1, following.1 + 1);
    }
    if leading.0 < following.0 && leading.1 < following.1 {
        return (following.0 - 1, following.1 - 1);
    }
    if leading.0 > following.0 && leading.1 < following.1 {
        return (following.0 + 1, following.1 - 1);
    }
    if leading.0 < following.0 && leading.1 > following.1 {
        return (following.0 - 1, following.1 + 1);
    }

    panic!("Shouldn't get here.");
}

fn next_head_positions(line: &str, h: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    let mut i = line.split_ascii_whitespace();
    let direction = i.next().unwrap();
    let delta: (i64, i64) = match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Unexpected!"),
    };

    let how_many = i.next().unwrap().parse::<i64>().unwrap();
    (1..(how_many + 1)).map(move |s| (h.0 + delta.0 * s, h.1 + delta.1 * s))
}

fn count_distinct_last_knot_positions<'a, T: Iterator<Item=&'a str>>(n_knots: usize, lines: T) -> usize {
    let mut knot_positions: Vec<(i64,i64)> = vec![(0,0); n_knots];
    let mut last_knot_positions: HashSet<(i64,i64)> = HashSet::new();
    last_knot_positions.insert(knot_positions[n_knots-1]);

    for line in  lines {
        for h_ in next_head_positions(line, knot_positions[0]) {
            knot_positions[0] = h_;
            for i in 1..n_knots {
                knot_positions[i] = next_following_knot_position(knot_positions[i], knot_positions[i-1]);
            }
            last_knot_positions.insert(knot_positions[n_knots-1]);
        }
    }

    last_knot_positions.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();
    println!("Unique tail positions [part 1]: {}", count_distinct_last_knot_positions(2, lines.clone()));
    println!("Unique tail positions [part 2]: {}", count_distinct_last_knot_positions(10, lines));
}
