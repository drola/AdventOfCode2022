/// Solution to an Advent of Code problem, day 11, 2022
/// https://adventofcode.com/2022/day/11
use nom::combinator::value;
use std::cmp::Reverse;
use std::env;
use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::{newline, space0, u64};
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Debug, PartialEq)]
enum Operand {
    Old,
    Number(u64),
}

#[derive(Clone, Debug, PartialEq)]
enum Test {
    DivisibleBy(u64),
}

#[derive(Clone, Debug, PartialEq)]
struct Operation {
    operator: Operator,
    left: Operand,
    right: Operand,
}

#[derive(Clone, Debug, PartialEq)]
enum Action {
    ThrowToMonkey(u64),
}

#[derive(Clone, Debug, PartialEq)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    action_if_true: Action,
    action_if_false: Action,
    inspections_count: u64,
}

// Input example:
//   Starting items: 74, 60, 97
fn parse_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, (_, _, items)) = tuple((
        space0,
        tag("Starting items: "),
        separated_list0(tag(", "), u64),
    ))(input)?;

    Ok((input, items))
}

// Input example:
// old
fn parse_operand_old(input: &str) -> IResult<&str, Operand> {
    let (input, _) = tag("old")(input)?;
    Ok((input, Operand::Old))
}

// Input example:
// 1234
fn parse_operand_u64(input: &str) -> IResult<&str, Operand> {
    let (input, n) = u64(input)?;
    Ok((input, Operand::Number(n)))
}

// Input examples:
// 1234
// old
fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((parse_operand_old, parse_operand_u64))(input)
}

// Input examples:
// +
// -
fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Add, tag("+")),
        value(Operator::Multiply, tag("*")),
    ))(input)
}

// Input examples:
//   Operation: new = old * old
//   Operation: new = old + 6
fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (_, _, left, _, operator, _, right)) = tuple((
        space0,
        tag("Operation: new = "),
        parse_operand,
        space0,
        parse_operator,
        space0,
        parse_operand,
    ))(input)?;
    Ok((
        input,
        Operation {
            operator,
            left,
            right,
        },
    ))
}

// Input example:
//    Test: divisible by 19
fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, (_, _, _, n)) = tuple((space0, tag("Test: "), tag("divisible by "), u64))(input)?;

    Ok((input, Test::DivisibleBy(n)))
}

// Input example:
//  throw to monkey 2
fn parse_action(input: &str) -> IResult<&str, Action> {
    let (input, (_, _, n)) = tuple((space0, tag("throw to monkey "), u64))(input)?;
    Ok((input, Action::ThrowToMonkey(n)))
}

// Input example:
// Monkey 2:
//   Starting items: 79, 60, 97
//   Operation: new = old * old
//   Test: divisible by 13
//     If true: throw to monkey 1
//     If false: throw to monkey 3
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((tag("Monkey "), u64::<&str, Error<&str>>, tag(":"), newline))(input)?;
    let (input, (items, _)) = tuple((parse_items, newline))(input)?;
    let (input, (operation, _)) = tuple((parse_operation, newline))(input)?;
    let (input, (test, _)) = tuple((parse_test, newline))(input)?;
    let (input, (_, _, action_if_true, _)) =
        tuple((space0, tag("If true: "), parse_action, newline))(input)?;
    let (input, (_, _, action_if_false)) = tuple((space0, tag("If false: "), parse_action))(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            action_if_true,
            action_if_false,
            inspections_count: 0,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(multispace0, parse_monkey)(input)
}

fn measure_monkey_business<F: FnMut(u64) -> u64>(
    rounds: u64,
    mut worry_manager: F,
    initial_state: &Vec<Monkey>,
) -> u64 {
    let mut monkeys: Vec<Monkey> = initial_state.to_vec();

    for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            for original_worry_level in monkeys[monkey_index].items.clone() {
                let left = match &monkeys[monkey_index].operation.left {
                    Operand::Old => original_worry_level,
                    Operand::Number(n) => *n,
                };
                let right = match &monkeys[monkey_index].operation.right {
                    Operand::Old => original_worry_level,
                    Operand::Number(n) => *n,
                };
                let mut worry_level = match &monkeys[monkey_index].operation.operator {
                    Operator::Add => left + right,
                    Operator::Multiply => left * right,
                };

                worry_level = worry_manager(worry_level);

                let Test::DivisibleBy(divisor) = monkeys[monkey_index].test;
                let Action::ThrowToMonkey(target_if_true) = monkeys[monkey_index].action_if_true;
                let Action::ThrowToMonkey(target_if_false) = monkeys[monkey_index].action_if_false;

                if worry_level % divisor == 0 {
                    monkeys[target_if_true as usize].items.push(worry_level);
                } else {
                    monkeys[target_if_false as usize].items.push(worry_level);
                }
            }

            monkeys[monkey_index].inspections_count =
                monkeys[monkey_index].inspections_count + monkeys[monkey_index].items.len() as u64;
            monkeys[monkey_index].items.clear();
        }
    }

    let mut inspections_counts = monkeys
        .iter()
        .map(|m| m.inspections_count)
        .collect::<Vec<u64>>();
    inspections_counts.sort_unstable_by_key(|&v| Reverse(v));
    return inspections_counts[0] * inspections_counts[1];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");

    let (_, monkeys) = parse_monkeys(&contents).unwrap();

    println!("{:?}", monkeys);

    println!(
        "Monkey business [part 1]: {}",
        measure_monkey_business(20, |w| w / 3, &monkeys)
    );

    let least_common_multiple: u64 = monkeys
        .iter()
        .map(|m| match m.test {
            Test::DivisibleBy(n) => n,
        })
        .product();
    println!(
        "Monkey business [part 2]: {}",
        measure_monkey_business(10000, |w| w % least_common_multiple, &monkeys)
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_items() {
        assert_eq!(
            parse_items("  Starting items: 54, 65, 75, 74"),
            Ok(("", vec![54, 65, 75, 74]))
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("  Operation: new = old + 6"),
            Ok((
                "",
                Operation {
                    operator: Operator::Add,
                    left: Operand::Old,
                    right: Operand::Number(6)
                }
            ))
        );
        assert_eq!(
            parse_operation("  Operation: new = old * old"),
            Ok((
                "",
                Operation {
                    operator: Operator::Multiply,
                    left: Operand::Old,
                    right: Operand::Old
                }
            ))
        );
    }

    #[test]
    fn test_parse_test() {
        assert_eq!(
            parse_test("  Test: divisible by 13"),
            Ok(("", Test::DivisibleBy(13)))
        );
    }

    #[test]
    fn test_parse_action() {
        assert_eq!(
            parse_action("throw to monkey 0"),
            Ok(("", Action::ThrowToMonkey(0)))
        );
    }
}
