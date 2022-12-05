/// Solution to an Advent of Code problem, day 05, 2022
/// https://adventofcode.com/2022/day/05
use std::env;
use std::fs;

fn show(stacks: &Vec<Vec<u8>>) {
    let depth = stacks.iter().map(|s| s.len()).max().unwrap_or(0);
    println!("");
    for i in 0..depth {
        for stack in stacks {
            if stack.len() > (depth - i - 1) {
                print!(
                    "[{}] ",
                    String::from_utf8(vec![stack[depth - i - 1]]).unwrap()
                );
            } else {
                print!("    ");
            }
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let mut lines = contents.lines().peekable();

    let first_line_length = lines.peek().unwrap().len();
    let stacks_count = (first_line_length + 1) / 4;
    let mut stacks: Vec<Vec<u8>> = vec![vec![]; stacks_count];

    loop {
        let line = lines.next().unwrap();

        if lines.peek().unwrap().len() < 1 {
            break;
        }

        for (idx, column) in line.as_bytes().chunks(4).enumerate() {
            let letter = column[1];
            if letter.is_ascii_uppercase() {
                stacks[idx].push(letter);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    //show(&stacks);

    lines.next(); // Skip empty line.

    let mut moves: Vec<(usize, usize, usize)> = vec![];

    for m in lines {
        let ascii_zero: u8 = 48;
        let mut words = m.split(' ');

        words.next(); // "move"

        let how_many = words.next().unwrap().parse::<usize>().unwrap();

        words.next(); // "from"
        let from = words.next().unwrap().parse::<usize>().unwrap() - 1;
        words.next(); // "to"
        let to = words.next().unwrap().parse::<usize>().unwrap() - 1;

        moves.push((how_many, from, to));

        // for _ in 0..how_many {
        //     println!("From {} to {}", from, to);
        //     let item = stacks[from as usize].pop().unwrap();
        //     stacks[to as usize].push(item);
        // }
        //show(&stacks);
    }

    let mut stacks_p1 = stacks.clone();
    for (how_many, from, to) in moves.clone() {
        for _ in 0..how_many {
            let item = stacks_p1[from].pop().unwrap();
            stacks_p1[to].push(item);
        }
    }
    let tops_p1 = stacks_p1
        .into_iter()
        .map(|s| s[s.len() - 1])
        .collect::<Vec<u8>>();
    println!("[part 1] Message: {}", String::from_utf8(tops_p1).unwrap());
    
    let mut stacks_p2 = stacks.clone();
    for (how_many, from, to) in moves {
        let split_index = stacks_p2[from].len() - how_many;
        let mut crates = stacks_p2[from].split_off(split_index);
        stacks_p2[to].append(&mut crates);
    }
    let tops_p2 = stacks_p2
        .into_iter()
        .map(|s| s[s.len() - 1])
        .collect::<Vec<u8>>();
    println!("[part 2] Message: {}", String::from_utf8(tops_p2).unwrap());

}
