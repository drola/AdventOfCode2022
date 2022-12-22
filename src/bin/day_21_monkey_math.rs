#![feature(drain_filter)]
/// Solution to an Advent of Code problem, day 21, 2022
/// https://adventofcode.com/2022/day/21
use std::env;
use std::fs;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::{i64, space0};
use nom::combinator::value;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

type Name = [u8; 4];
const HUMN: Name = *b"humn";
const ROOT: Name = *b"root";
const BLANK: Name = *b"____";

#[derive(Clone, Debug)]
enum Operator {
    Plus,
    Minus,
    Times,
    DivideBy,
}

#[derive(Clone, Debug)]
enum Monkey {
    CalculatingMonkey {
        name: Name,
        left: Name,
        right: Name,
        operator: Operator,
    },
    ShoutingMonkey {
        name: Name,
        number: i64,
    },
    PausedMonkey {
        name: Name,
    },
}

fn parse_name(input: &str) -> IResult<&str, Name> {
    let (input, name) = take(4usize)(input)?;

    Ok((input, name.as_bytes().try_into().unwrap()))
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Plus, tag("+")),
        value(Operator::Minus, tag("-")),
        value(Operator::Times, tag("*")),
        value(Operator::DivideBy, tag("/")),
    ))(input)
}

fn parse_calculating_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (name, _, left, _, operator, _, right)) = tuple((
        parse_name,
        tag(": "),
        parse_name,
        space0,
        parse_operator,
        space0,
        parse_name,
    ))(input)?;

    return Ok((
        input,
        Monkey::CalculatingMonkey {
            name,
            left,
            right,
            operator,
        },
    ));
}
fn parse_shouting_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (name, _, number)) = tuple((parse_name, tag(": "), i64))(input)?;
    return Ok((input, Monkey::ShoutingMonkey { name, number }));
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    alt((parse_calculating_monkey, parse_shouting_monkey))(input)
}

fn evaluate(monkeys: &mut Vec<Monkey>) -> HashMap<Name, i64> {
    let mut something_evaluated = true;
    let mut evaluated = HashMap::new();
    while something_evaluated {
        let monkeys_to_evaluate = monkeys
            .drain_filter(|m| match m {
                Monkey::ShoutingMonkey { name: _, number: _ } => true,
                Monkey::CalculatingMonkey {
                    name: _,
                    left,
                    right,
                    operator: _,
                } => evaluated.contains_key(left) && evaluated.contains_key(right),
                Monkey::PausedMonkey { name: _ } => false,
            })
            .collect_vec();
        something_evaluated = monkeys_to_evaluate.len() > 0;
        for monkey in monkeys_to_evaluate {
            match monkey {
                Monkey::ShoutingMonkey { name, number } => {
                    evaluated.insert(name, number);
                }
                Monkey::CalculatingMonkey {
                    name,
                    left,
                    right,
                    operator,
                } => {
                    let left_ = evaluated.get(&left).unwrap();
                    let right_ = evaluated.get(&right).unwrap();
                    evaluated.insert(
                        name,
                        match operator {
                            Operator::Plus => left_ + right_,
                            Operator::Minus => left_ - right_,
                            Operator::Times => left_ * right_,
                            Operator::DivideBy => left_ / right_,
                        },
                    );
                }
                Monkey::PausedMonkey { name: _ } => {}
            }
        }
    }
    return evaluated;
}

fn evaluate_for_name(
    humn_value: i64,
    humn_index: usize,
    name: Name,
    initial_monkeys: &Vec<Monkey>,
) -> i64 {
    let mut monkeys = initial_monkeys.clone();
    monkeys[humn_index] = Monkey::ShoutingMonkey {
        name: HUMN,
        number: humn_value,
    };

    let cache = evaluate(&mut monkeys);
    *cache.get(&name).unwrap()
}

fn is_between(a: i64, b: i64, x: i64) -> bool {
    (a <= x && x < b) || (a > x && x >= b)
}

fn find_monkey_index(name_: Name, monkeys: &Vec<Monkey>) -> usize {
    monkeys
        .iter()
        .find_position(|m| match m {
            Monkey::CalculatingMonkey {
                name,
                left: _,
                right: _,
                operator: _,
            } => *name == name_,
            Monkey::ShoutingMonkey { name, number: _ } => *name == name_,
            Monkey::PausedMonkey { name } => *name == name_,
        })
        .unwrap()
        .0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let monkeys = contents
        .lines()
        .map(|f| parse_monkey(f).unwrap().1)
        .collect_vec();
    //println!("{:?}", monkeys);

    let mut monkeys_part_1 = monkeys.clone();
    let evaluated_part_1 = evaluate(&mut monkeys_part_1);
    println!("Root [part 1]: {}", evaluated_part_1.get(&ROOT).unwrap());

    // Part 2
    let mut monkeys_part_2 = monkeys.clone();
    let humn_index = find_monkey_index(HUMN, &monkeys_part_2);
    let _root_index = find_monkey_index(ROOT, &monkeys_part_2);
    let mut root_monkey_left: Name = BLANK;
    let mut root_monkey_right: Name = BLANK;
    for m in monkeys_part_2.iter() {
        match m {
            Monkey::CalculatingMonkey {
                name,
                left,
                right,
                operator: _,
            } if *name == ROOT => {
                root_monkey_left = *left;
                root_monkey_right = *right;
            }
            _ => {}
        }
    }
    monkeys_part_2[humn_index] = Monkey::PausedMonkey { name: HUMN };
    //let cache = evaluate(&mut monkeys_part_2, HashMap::new());

    // Both, in test and real input, the "humn" is in the left side of the equation.
    // Our target value is on the right side.
    let target_value = evaluate_for_name(0, humn_index, root_monkey_right, &monkeys_part_2);

    let humn_index = find_monkey_index(HUMN, &monkeys_part_2);

    // Bisection ahead --->>>>>>

    // Initial search space (determined by trial and error)
    let mut a = 0;
    let mut b = 5000000000000;

    loop {
        let c = (a + b) / 2;
        let for_a = evaluate_for_name(a, humn_index, root_monkey_left, &monkeys_part_2);
        let for_b = evaluate_for_name(b, humn_index, root_monkey_left, &monkeys_part_2);
        let for_c = evaluate_for_name(c, humn_index, root_monkey_left, &monkeys_part_2);

        if for_a == target_value {
            println!("Humn [part 2]: {}", a);
            break;
        } else if for_b == target_value {
            println!("Humn [part 2]: {}", b);
            break;
        } else if is_between(for_a, for_c, target_value) {
            b = c;
        } else if is_between(for_c, for_b, target_value) {
            a = c;
        } else {
            panic!("Search space too narrow!");
        }

        //println!("a={}, b={}", a, b);
    }
}
