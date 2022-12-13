/// Solution to an Advent of Code problem, day 13, 2022
/// https://adventofcode.com/2022/day/13
use std::env;
use std::fs;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u64;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Number(u64),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::List(_), Packet::Number(b)) => {
                self.cmp(&Packet::List(vec![Packet::Number(*b)]))
            }
            (Packet::Number(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*a)]).cmp(other)
            }
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
        }
    }
}

fn parse_list(input: &str) -> IResult<&str, Packet> {
    let (input, (_, a, _)) =
        tuple((tag("["), separated_list0(tag(","), parse_packet), tag("]")))(input)?;
    Ok((input, Packet::List(a)))
}

fn parse_number(input: &str) -> IResult<&str, Packet> {
    map(u64, |n| Packet::Number(n))(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((parse_number, parse_list))(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let lines = contents.lines();

    let mut sum_of_ordered_indices: u64 = 0;
    let mut all_packets: Vec<Packet> = vec![];

    for (pair_index, mut pair) in lines.chunks(3).into_iter().enumerate() {
        let a = parse_packet(pair.next().unwrap()).unwrap().1;
        let b = parse_packet(pair.next().unwrap()).unwrap().1;

        // println!("{:?}", a);
        // println!("{:?}", b);
        // println!("");

        if a < b {
            sum_of_ordered_indices = sum_of_ordered_indices + pair_index as u64 + 1;
        }

        all_packets.push(a);
        all_packets.push(b);
    }

    println!(
        "Sum of ordered indices [part 1]: {}",
        sum_of_ordered_indices
    );


    // Insert divider packets
    let divider_packet_1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let divider_packet_2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    all_packets.push(divider_packet_1.clone());
    all_packets.push(divider_packet_2.clone());

    all_packets.sort();

    let decoder_key: u64 = all_packets
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v == &divider_packet_1 || v == &divider_packet_2 {
                Some(i as u64 + 1)
            } else {
                None
            }
        })
        .product();

    println!("Decoder key [part 2]: {}", decoder_key);
}
