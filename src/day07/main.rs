use std::fs;

type Input = Vec<i64>;
type Input1 = Input;
type Input2 = Input;
const INPUT_FILE: &str = "inputs/07.txt";

fn median(input: &Input) -> i64 {
    let mut input = input.clone();
    input.sort();
    input[input.len() / 2]
}

fn val(n: i64) -> i64 {
    (n + 1) * n / 2
}

fn part1(input: &Input1) -> String {
    let med = median(input);
    let diff = input.iter().map(|x| (x - med).abs()).sum::<i64>();
    format!("{:?}", diff)
}

fn part2(input: &Input2) -> String {
    // I'm sure there's a smart way to do this
    // It's very close to the part1 approach but with the mean, so I'm probably just messing something up there
    let min = (0..input.len())
        .map(|x| input.iter().map(|y| val((x as i64 - y).abs())).sum::<i64>())
        .min()
        .unwrap();
    format!("{:?}", min)
}

fn parse_input_1(lines: &Vec<String>) -> Input1 {
    parse_input(lines)
}

fn parse_input_2(lines: &Vec<String>) -> Input2 {
    parse_input(lines)
}

fn parse_input(lines: &Vec<String>) -> Input {
    lines
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
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
