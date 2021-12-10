use std::fs;

type Input = Vec<Vec<char>>;
type Input1 = Input;
type Input2 = Input;

const INPUT_FILE: &str = "inputs/10.txt";

const OPEN_1: char = '(';
const OPEN_2: char = '[';
const OPEN_3: char = '{';
const OPEN_4: char = '<';
const OPENS: [char; 4] = [OPEN_1, OPEN_2, OPEN_3, OPEN_4];
const CLOSE_1: char = ')';
const CLOSE_2: char = ']';
const CLOSE_3: char = '}';
const CLOSE_4: char = '>';
const CLOSES: [char; 4] = [CLOSE_1, CLOSE_2, CLOSE_3, CLOSE_4];

fn matching_pair(c: char) -> char {
    match c {
        OPEN_1 => CLOSE_1,
        OPEN_2 => CLOSE_2,
        OPEN_3 => CLOSE_3,
        OPEN_4 => CLOSE_4,
        _ => panic!("Invalid character"),
    }
}

fn close_score(c: char) -> i32 {
    match c {
        CLOSE_1 => 3,
        CLOSE_2 => 57,
        CLOSE_3 => 1197,
        CLOSE_4 => 25137,
        _ => panic!("Invalid character"),
    }
}

// returns the remaining stack, and the invalid character, if any
fn process_line(line: &Vec<char>) -> (Vec<char>, Option<char>) {
    let mut stack = Vec::new();

    for char in line.iter() {
        if OPENS.contains(char) {
            stack.push(*char);
        } else if CLOSES.contains(char) {
            if stack.is_empty() {
                return (stack, Some(*char));
            }
            let last = stack.pop().unwrap();
            if *char != matching_pair(last) {
                stack.push(last);
                return (stack, Some(*char));
            }
        }
    }

    (stack, None)
}

fn part1(input: &Input1) -> String {
    let mut sum = 0;
    for line in input.iter() {
        if let (_, Some(char)) = process_line(line) {
            sum += close_score(char);
        }
    }
    sum.to_string()
}

fn open_score(c: char) -> i128 {
    match c {
        OPEN_1 => 1,
        OPEN_2 => 2,
        OPEN_3 => 3,
        OPEN_4 => 4,
        _ => panic!("Invalid character"),
    }
}

fn part2(input: &Input2) -> String {
    let mut scores: Vec<i128> = Vec::new();
    for line in input.iter() {
        if let (mut stack, None) = process_line(line) {
            let mut score: i128 = 0;
            stack.reverse();
            for c in stack.iter() {
                score *= 5;
                score += open_score(*c);
            }
            scores.push(score);
        }
    }
    scores.sort();
    scores[scores.len() / 2].to_string()
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = Vec::new();
    for line in lines.iter() {
        input.push(line.chars().collect());
    }
    input
}

fn parse_input_1(lines: &Vec<String>) -> Input1 {
    parse_input(lines)
}

fn parse_input_2(lines: &Vec<String>) -> Input2 {
    parse_input(lines)
}

fn lines(filename: &str) -> Vec<String> {
    let file_string = fs::read_to_string(filename).expect("Something went wrong reading the file");
    file_string
        .split("\n")
        .map(|x| x.trim().to_string())
        .collect()
}

fn main() {
    let lines = lines(INPUT_FILE);

    println!("{}", part1(&parse_input_1(&lines)));
    println!("{}", part2(&parse_input_2(&lines)));
}
