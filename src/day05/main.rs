use std::collections::HashMap;
use std::fs;

type Input = Vec<Vent>;
type Input1 = Input;
type Input2 = Input;
const INPUT_FILE: &str = "inputs/05.txt";

fn count_collisions(input: &Input, allow_diagonal: bool) -> usize {
    let mut coords: HashMap<(usize, usize), usize> = HashMap::new();
    let mut collisions: usize = 0;

    for vent in input {
        if vent.is_diagonal() && !allow_diagonal {
            continue;
        }

        for coord in vent.iter() {
            let entry = coords.entry(coord).or_insert(0);
            if *entry == 1 {
                collisions += 1;
            }
            *entry += 1;
        }
    }

    collisions
}

fn part1(input: &Input1) -> String {
    format!("{:?}", count_collisions(input, false))
}

fn part2(input: &Input2) -> String {
    format!("{:?}", count_collisions(input, true))
}

#[derive(Debug, Clone)]
struct Vent {
    start: (usize, usize),
    end: (usize, usize),
}

impl Vent {
    fn is_diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let (x0, y0) = self.start;
        let (x1, y1) = self.end;
        let dx: i16 = match x1.cmp(&x0) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        let dy: i16 = match y1.cmp(&y0) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        };
        let mut x = x0;
        let mut y = y0;
        let mut done = false;

        std::iter::from_fn(move || {
            // This is kind of dumb, but getting the ends to be inclusive is annoying
            if done {
                None
            } else if x == x1 && y == y1 {
                done = true;
                Some((x, y))
            } else {
                let ret = (x, y);
                x = (x as i16 + dx) as usize;
                y = (y as i16 + dy) as usize;
                Some(ret)
            }
        })
    }
}

fn parse_input_1(lines: &Vec<String>) -> Input1 {
    parse_input(lines)
}

fn parse_input_2(lines: &Vec<String>) -> Input2 {
    parse_input(lines)
}

fn parse_line(line: &String) -> Vent {
    let mut parts = line.split(" -> ");

    let mut start_part = parts.next().unwrap().split(",");
    let mut end_part = parts.next().unwrap().split(",");
    let start = (
        start_part.next().unwrap().parse::<usize>().unwrap(),
        start_part.next().unwrap().parse::<usize>().unwrap(),
    );

    let end = (
        end_part.next().unwrap().parse::<usize>().unwrap(),
        end_part.next().unwrap().parse::<usize>().unwrap(),
    );

    Vent { start, end }
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = Vec::new();
    for line in lines {
        input.push(parse_line(line))
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
