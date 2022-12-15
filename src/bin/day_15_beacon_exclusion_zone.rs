use std::cmp::max;
use std::cmp::min;
/// Solution to an Advent of Code problem, day 15, 2022
/// https://adventofcode.com/2022/day/15
use std::env;
use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Copy, Clone)]
struct Sensor {
    pos: (i64, i64),
    closest_beacon: (i64, i64),
}
fn parse_coordinates(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, (_, x, _, y)) = tuple((tag("x="), i64, tag(", y="), i64))(input)?;
    Ok((input, (x, y)))
}
fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, (_, pos, _, closest_beacon)) = tuple((
        tag("Sensor at "),
        parse_coordinates,
        tag(": closest beacon is at "),
        parse_coordinates,
    ))(input)?;

    Ok((
        input,
        Sensor {
            pos,
            closest_beacon,
        },
    ))
}

fn row_coverage(sensor: &Sensor, y: i64) -> Option<(i64, i64)> {
    let mhd = (sensor.pos.0 - sensor.closest_beacon.0).abs()
        + (sensor.pos.1 - sensor.closest_beacon.1).abs();

    let dy = (y - sensor.pos.1).abs();
    if dy > mhd {
        return None;
    }
    return Some((sensor.pos.0 - (mhd - dy), (sensor.pos.0 + (mhd - dy))));
}

fn do_intervals_overlap(a: (i64, i64), b: (i64, i64)) -> bool {
    a.1 >= b.0 || b.1 >= a.0
}

fn is_in_interval(i: (i64, i64), p: i64) -> bool {
    p >= i.0 && p <= i.1
}

fn interval_intersection(a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    if do_intervals_overlap(a, b) {
        Some((max(a.0, b.0), min(a.1, b.1)))
    } else {
        None
    }
}

fn accumulate_intervals(intervals: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut endpoints: Vec<i64> = vec![];
    for interval in intervals.iter() {
        endpoints.push(interval.0);
        endpoints.push(interval.1);
        endpoints.push(interval.1 + 1);
    }
    endpoints.sort();
    endpoints.dedup();

    let is_in = endpoints
        .iter()
        .map(|&p| intervals.iter().any(|&i| is_in_interval(i, p)))
        .collect::<Vec<bool>>();

    //println!("{:?}", endpoints);
    //println!("{:?}", is_in);

    let mut result: Vec<(i64, i64)> = vec![];
    if endpoints.len() >= 2 {
        for i in 0..endpoints.len() - 1 {
            if is_in[i] && is_in[i + 1] {
                if result.len() > 0 && result[result.len() - 1].1 == endpoints[i] {
                    result.last_mut().unwrap().1 = endpoints[i + 1];
                } else {
                    result.push((endpoints[i], endpoints[i + 1]));
                }
            }
        }
    }
    result
}

fn find_uncovered_spot(intervals: &Vec<(i64, i64)>, range_min: i64, range_max: i64) -> Option<i64> {
    let first_overlapping_interval = accumulate_intervals(intervals)
        .iter()
        .filter_map(|&i| interval_intersection(i, (range_min, range_max)))
        .next();

    match (first_overlapping_interval) {
        Some((a, b)) if a <= range_min && b >= range_max => None, // Complete overlap
        Some((a, b)) if b < range_max => Some(b + 1),
        Some((a, b)) if a < range_min => Some(a - 1),
        _ => Some(range_min),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let sensors = contents
        .lines()
        .map(|l| parse_sensor(l).unwrap().1)
        .collect::<Vec<Sensor>>();
    println!("{:?}", sensors);

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    for s in sensors.iter() {
        min_x = min(min_x, min(s.pos.0, s.closest_beacon.0));
        max_x = max(max_x, max(s.pos.0, s.closest_beacon.0));
        min_y = min(min_y, min(s.pos.1, s.closest_beacon.1));
        max_y = max(max_y, max(s.pos.1, s.closest_beacon.1));
    }

    println!(
        "Coord ranges: X: [{}, {}]; Y: [{}, {}]",
        min_x, max_x, min_y, max_y
    );
    println!(
        "Dimensions: W={}, H={}, Cell count={}",
        max_x - min_x,
        max_y - min_y,
        (max_x - min_x) * (max_y - min_y)
    );

    let interesting_row = 2000000;

    let max_mhd = sensors
        .iter()
        .map(|s| (s.pos.0 - s.closest_beacon.0).abs() + (s.pos.1 - s.closest_beacon.1).abs())
        .max()
        .unwrap();

    let mut row = vec![true; (max_x - min_x + 1 + 2 * max_mhd) as usize];
    for sensor in sensors.iter() {
        if let Some((x0, x1)) = row_coverage(sensor, interesting_row) {
            for x in x0..x1 + 1 {
                row[(x - min_x + max_mhd) as usize] = false;
            }
        }
    }

    // Remove known beacons
    for sensor in sensors.iter() {
        if sensor.closest_beacon.1 == interesting_row {
            row[(sensor.closest_beacon.0 - min_x + max_mhd) as usize] = true;
        }
    }

    let count = row.iter().filter(|&&v| !v).count();
    println!("Count: {}", count);

    let search_space = 4000000;
    for y in 0..search_space + 1 {
        // Here we could avoid allocating Vec<> in each loop. We could probably get away with passing around iterators.
        let pos = find_uncovered_spot(
            &sensors.iter().filter_map(|s| row_coverage(s, y)).collect::<Vec<(i64,i64)>>(),
            0,
            search_space,
        );

        if let Some(x) = pos {
            println!("Distress from x={}, y={}; Frequency = {}", x, y, x*4000000+y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulate_intervals() {
        // Trivial case
        assert_eq!(accumulate_intervals(&vec![]), vec![]);

        assert_eq!(
            accumulate_intervals(&vec![(0, 13), (15, 20)]),
            vec![(0, 13), (15, 20)]
        );
        assert_eq!(
            accumulate_intervals(&vec![(0, 14), (15, 20)]),
            vec![(0, 20)]
        );
        assert_eq!(
            accumulate_intervals(&vec![(0, 15), (15, 20)]),
            vec![(0, 20)]
        );
        assert_eq!(
            accumulate_intervals(&vec![(0, 15), (15, 20), (-5, -2)]),
            vec![(-5, -2), (0, 20)]
        );
        assert_eq!(
            accumulate_intervals(&vec![(0, 15), (15, 20), (-5, -2), (-2, -1)]),
            vec![(-5, 20)]
        );
    }

    #[test]
    fn test_find_uncovered_spot() {
        assert_eq!(find_uncovered_spot(&vec![], 0, 10), Some(0));
        assert_eq!(find_uncovered_spot(&vec![(0, 10)], 0, 10), None);
        assert_eq!(find_uncovered_spot(&vec![(0, 10)], 0, 11), Some(11));
        assert_eq!(find_uncovered_spot(&vec![(0, 4), (5, 10)], 0, 10), None);
    }
}
