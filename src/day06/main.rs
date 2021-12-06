use std::fs;

type Input = [u128; 9];
type Input1 = Input;
type Input2 = Input;
const INPUT_FILE: &str = "inputs/06.txt";

fn step(input: &mut Input) {
    let next = input[0];
    for i in 0..(input.len() - 1) {
        input[i] = input[i + 1];
    }
    input[6] += next;
    input[8] = next;
}

fn step_n_count(input: &Input, n: usize) -> u128 {
    let mut input = input.clone();
    for _ in 0..n {
        step(&mut input);
    }
    input.into_iter().sum()
}

fn part1(input: &Input1) -> String {
    format!("{:?}", step_n_count(&input, 80))
}

fn part2(input: &Input2) -> String {
    format!("{:?}", step_n_count(&input, 256))
}

fn parse_input_1(lines: &Vec<String>) -> Input1 {
    parse_input(lines)
}

fn parse_input_2(lines: &Vec<String>) -> Input2 {
    parse_input(lines)
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = [0; 9];
    for num in lines
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
    {
        input[num] += 1;
    }
    input
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
