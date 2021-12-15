#![feature(map_first_last)]

use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Debug, Clone)]
struct Input {
    grid: Vec<Vec<u32>>,
}

type Input1 = Input;
type Input2 = Input;

const INPUT_FILE: &str = "inputs/15.txt";

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    pos: (usize, usize),
    score: u32,
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
            .then_with(|| self.pos.cmp(&other.pos)) // this is backwards to get correct min heap ordering
        //self.score.cmp(&other.score)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Input {
    fn new(lines: &Vec<String>) -> Self {
        let mut grid = Vec::new();
        for line in lines.iter() {
            grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }
        Input { grid }
    }

    fn scale_up(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        for i in 0..4 {
            for row in 0..rows {
                let new_row = self.grid[row].iter().map(|c| (c + i) % 9 + 1).collect::<Vec<u32>>();
                self.grid.push(new_row);
            }
        }

        for i in 0..4 {
            for row in 0..self.grid.len() {
                for col in 0..cols {
                    let new_val = (self.grid[row][col] + i) % 9 + 1;
                    self.grid[row].push(new_val);
                }
            }
        }
    }

    fn djikstra(&self) -> u32 {
        let mut scores: HashMap<(usize, usize), u32> = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(State {
            pos: (0, 0),
            score: 0,
        });
        while let Some(next) = queue.pop() {
            if next.pos == (self.grid.len() - 1, self.grid[0].len() - 1) {
                return next.score;
            }
            for neighbor in self.neighbors(next.pos) {
                let new_score = next.score + self.grid[neighbor.1][neighbor.0];
                if let Some(score) = scores.get(&neighbor) {
                    if *score <= new_score {
                        continue;
                    }
                }
                queue.push(State {
                    pos: neighbor,
                    score: new_score,
                });
                scores.insert(neighbor, new_score);
            }
        }
        panic!("No path found");
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.grid[0].len() - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.grid.len() - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }
}

fn part1(input: &Input1) -> String {
    format!("{:?}", input.djikstra())
}

fn part2(input: &Input2) -> String {
    let mut input = input.clone();
    input.scale_up();
    format!("{:?}", input.djikstra())
}

fn parse_input(lines: &Vec<String>) -> Input {
    Input::new(lines)
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
