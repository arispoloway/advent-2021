use std::fs;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Forward,
    Down,
    Up
}

impl Direction {
    fn parse(dir: &str) -> Direction {
        if dir == "forward" {
            Direction::Forward {}
        } else if dir == "down" {
            Direction::Down {}
        } else if dir == "up" {
            Direction::Up {}
        } else {
            panic!("Unrecognized direction")
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Action {
    direction: Direction,
    amount: i32
}

fn part1(actions: &Vec<Action>) -> i64 {
    let mut distance: i64 = 0;
    let mut depth: i64 = 0;

    for action in actions {
        match action {
            Action { direction: Direction::Up, amount } => depth = depth - (*amount as i64),
            Action { direction: Direction::Down, amount } => depth = depth + (*amount as i64),
            Action { direction: Direction::Forward, amount } => distance = distance + (*amount as i64),
        }
    }

    distance * depth
}

fn part2(actions: &Vec<Action>) -> i64 {
    let mut aim: i64 = 0;
    let mut distance: i64 = 0;
    let mut depth: i64 = 0;

    for action in actions {
        match action {
            Action { direction: Direction::Up, amount } => aim = aim - (*amount as i64),
            Action { direction: Direction::Down, amount } => aim = aim + (*amount as i64),
            Action { direction: Direction::Forward, amount } => {
                distance = distance + (*amount as i64);
                depth = depth + (aim * (*amount as i64));
            },
        }
    }

    distance * depth
}

fn main() {
    let actions = parse_input();

    println!("{}", part1(&actions));
    println!("{}", part2(&actions));
}

fn parse_input() -> Vec<Action> {
    parse_lines(lines("inputs/02.txt"))
}

fn parse_lines(lines: Vec<String>) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();

    for line in lines {
        let test: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
        let direction = Direction::parse(&test[0]);
        let amount = test[1].parse::<i32>().unwrap();
        let action = Action { direction, amount };
        actions.push(action);
    }

    actions
}

fn lines(filename: &str) -> Vec<String> {
    let file_string = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    file_string
        .split("\n")
        .map(|x| x.trim_end().to_string())
        .collect()
}
