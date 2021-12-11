#![feature(map_first_last)]

use std::collections::BTreeSet;
use std::fs;

#[derive(Debug, Clone)]
struct Input {
    grid: [[u8; 10]; 10],
}

type Input1 = Input;
type Input2 = Input;

const NEIGHBORS: [(i8, i8); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl<'a> Input {
    fn new(grid: [[u8; 10]; 10]) -> Self {
        Input { grid }
    }

    fn iter(&self) -> impl Iterator<Item=Coord> {
        (0..10).flat_map(move |y| (0..10).map(move |x| (x, y)))
    }

    // inc the value of the given coord, returning the new value
    fn inc(&mut self, (x, y): &Coord) -> u8 {
        self.grid[*y][*x] += 1;
        self.grid[*y][*x]
    }

    fn reset(&mut self, (x, y): &Coord) {
        self.grid[*y][*x] = 0;
    }

    fn get(&mut self, (x, y): &Coord) -> u8 {
        self.grid[*y][*x]
    }

    fn neighbors((x, y): &'a Coord) -> impl Iterator<Item=Coord> + 'a {
        NEIGHBORS.iter().map(|&(dx, dy)| (*x as i8 + dx, *y as i8 + dy)).filter(|&(x, y)| x >= 0 && y >= 0 && x < 10 && y < 10).map(|(x, y)| (x as usize, y as usize))
    }

    // return the number of flashes
    fn step(&mut self) -> usize {
        let mut flashed = BTreeSet::new();
        let mut will_flash = BTreeSet::new();
        for coord in self.iter() {
            if self.inc(&coord) == 10 {
                flashed.insert(coord);
                will_flash.insert(coord);
            }
        }

        while let Some(coord) = will_flash.pop_first() {
            for neighbor in Input::neighbors(&coord) {
                if !flashed.contains(&neighbor) {
                    if self.inc(&neighbor) == 10 {
                        flashed.insert(neighbor);
                        will_flash.insert(neighbor);
                    }
                }
            }
        }

        for coord in self.iter() {
            if self.get(&coord) > 9 {
                self.reset(&coord)
            }
        }

        flashed.len()
    }
}

const INPUT_FILE: &str = "inputs/11.txt";

type Coord = (usize, usize);

fn part1(input: &Input1) -> String {
    let mut sum: usize = 0;
    let mut input = input.clone();
    for _ in 0..100 {
        sum += input.step() as usize;
    }
    sum.to_string()
}

fn part2(input: &Input2) -> String {
    let mut count: usize = 0;
    let mut input = input.clone();
    loop {
        count += 1;
        if input.step() == 100 {
            break;
        }
    }
    count.to_string()
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = [[0; 10]; 10];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            input[y][x] = c.to_digit(10).unwrap() as u8
        }
    }
    Input::new(input)
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
