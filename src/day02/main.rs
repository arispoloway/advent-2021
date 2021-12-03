use std::fs;

type Input = Vec<Action>;
const INPUT_FILE: &str = "inputs/02.txt";

fn parse_lines(lines: Vec<String>) -> Input {
    let mut actions: Vec<Action> = Vec::new();

    for line in lines {
        let action = Action::parse(&line);
        actions.push(action);
    }

    actions
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Forward(i32),
    Down(i32),
    Up(i32)
}

impl Action {
    fn parse(line: &String) -> Action {
        let mut split = line.split(" ");
        let dir = split.next().unwrap();
        let amount = split.next().unwrap().parse::<i32>().unwrap();
        if dir == "forward" {
            Action::Forward(amount)
        } else if dir == "down" {
            Action::Down(amount)
        } else if dir == "up" {
            Action::Up(amount)
        } else {
            panic!("Unrecognized direction")
        }
    }
}

fn part1(input: &Input) -> String {
    let mut distance: i64 = 0;
    let mut depth: i64 = 0;

    for action in input {
        match action {
            Action::Up(amount) => depth = depth - (*amount as i64),
            Action::Down(amount) => depth = depth + (*amount as i64),
            Action::Forward(amount) => distance = distance + (*amount as i64),
        }
    }

    format!("{}", distance * depth)
}

fn part2(input: &Input) -> String {
    let mut aim: i64 = 0;
    let mut distance: i64 = 0;
    let mut depth: i64 = 0;

    for action in input {
        match action {
            Action::Up(amount) => aim = aim - (*amount as i64),
            Action::Down(amount) => aim = aim + (*amount as i64),
            Action::Forward(amount) => {
                distance = distance + (*amount as i64);
                depth = depth + (aim * (*amount as i64));
            },
        }
    }

    format!("{}", distance * depth)
}

fn parse_input() -> Input {
    parse_lines(lines(INPUT_FILE))
}

fn lines(filename: &str) -> Vec<String> {
    let file_string = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    file_string
        .split("\n")
        .map(|x| x.trim_end().to_string())
        .collect()
}

fn main() {
    let actions = parse_input();

    println!("{}", part1(&actions));
    println!("{}", part2(&actions));
}