/// Solution to an Advent of Code problem, day 07, 2022
/// https://adventofcode.com/2022/day/07
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, not_line_ending, space1, u64};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
enum Line<'a> {
    Cd(&'a str),
    Dir(&'a str),
    File(&'a str, u64),
    Ls,
}

fn parse_cd(input: &str) -> IResult<&str, Line> {
    let (input, (_, d)) = tuple((tag("$ cd "), not_line_ending))(input)?;
    Ok((input, Line::Cd(d)))
}

fn parse_dir(input: &str) -> IResult<&str, Line> {
    let (input, (_, d)) = tuple((tag("dir "), not_line_ending))(input)?;
    Ok((input, Line::Dir(d)))
}

fn parse_file(input: &str) -> IResult<&str, Line> {
    let (input, (size, _, d)) = tuple((u64, space1, not_line_ending))(input)?;
    Ok((input, Line::File(d, size)))
}

fn parse_ls(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, Line::Ls))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((parse_cd, parse_dir, parse_file, parse_ls))(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let (_, lines) = separated_list1(line_ending, parse_line)(&contents).unwrap();
    // println!("{:?}", lines);

    let mut working_dir: Vec<&str> = vec![""];
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    for line in lines {
        match line {
            Line::Cd(d) => match d {
                "/" => {
                    working_dir.clear();
                    working_dir.push("");
                }
                ".." => {
                    working_dir.pop();
                }
                s => {
                    working_dir.push(s);
                }
            },
            Line::Dir(_d) => {} // Doesn't contribute to the result
            Line::File(_filename, size) => {
                let mut path = "".to_owned();
                for d in &working_dir {
                    path = path + d + "/";
                    dir_sizes.insert(path.to_string(), *dir_sizes.get(&path).unwrap_or(&0) + size);
                }
            }
            Line::Ls => {}
        }
    }

    // println!("{:?}", dir_sizes);
    // println!("");

    let sum_p1: u64 = dir_sizes.values().filter(|&&v| v <= 100000).sum();
    println!("[part 1] Dir sizes sum: {}", sum_p1);

    let total_disk_space: u64 = 70000000;
    let required_free_space: u64 = 30000000;
    let used_space: u64 = *dir_sizes.get("/").unwrap();

    let amount_to_delete = required_free_space + used_space - total_disk_space;
    let smallest_d: u64 = *dir_sizes
        .values()
        .filter(|&&v| v >= amount_to_delete)
        .min()
        .unwrap();
    println!(
        "[part 2] Size of the directory to be deleted: {}",
        smallest_d
    );
}
