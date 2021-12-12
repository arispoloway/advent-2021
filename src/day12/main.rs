#![feature(map_first_last)]

use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Input {
    edges: HashMap<String, HashSet<String>>,
}

type Input1 = Input;
type Input2 = Input;

const INPUT_FILE: &str = "inputs/12.txt";

impl Input {
    fn new() -> Self {
        Input {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let from = from.to_string();
        let to = to.to_string();
        self.edges.entry(from).or_insert(HashSet::new()).insert(to);
    }

    fn get_edges(&self, from: &String) -> &HashSet<String> {
        &self.edges[from]
    }
}

enum CaveType {
    Small,
    Big,
    Start,
    End,
}

fn cave_type(str: &str) -> CaveType {
    match str {
        "start" => CaveType::Start,
        "end" => CaveType::End,
        x if x.to_uppercase() == x => CaveType::Big,
        _ => CaveType::Small,
    }
}

// TODO: Fix up all of my .to_string() calls, as well as my probably bad representation
// of the graph which makes me need to call cave_type everywhere

fn count_to_end_1(input: &Input, from: String, visited_small_caves: &mut HashSet<String>) -> usize {
    let mut sum = 0;
    for next in input.get_edges(&from).iter() {
        let next_type = cave_type(&next);
        match next_type {
            CaveType::End => {
                sum += 1;
            }
            CaveType::Small => {
                if !visited_small_caves.contains(next) {
                    visited_small_caves.insert(next.to_string());
                    sum += count_to_end_1(input, next.to_string(), visited_small_caves);
                    visited_small_caves.remove(next);
                }
            }
            CaveType::Big => {
                sum += count_to_end_1(input, next.to_string(), visited_small_caves);
            }
            _ => { panic!("Should never go back to start"); }
        }
    }
    sum
}

fn count_to_end_2(input: &Input, from: String, visited_small_caves: &mut HashSet<String>, visited_twice: &Option<String>) -> usize {
    let mut sum = 0;
    for next in input.get_edges(&from).iter() {
        let next_type = cave_type(&next);
        match next_type {
            CaveType::End => {
                sum += 1;
            }
            CaveType::Small => {
                if !visited_small_caves.contains(next) {
                    visited_small_caves.insert(next.to_string());
                    sum += count_to_end_2(input, next.to_string(), visited_small_caves, visited_twice);
                    visited_small_caves.remove(next);
                } else if visited_twice.is_none() {
                    sum += count_to_end_2(input, next.to_string(), visited_small_caves, &Some(next.to_string()));
                }
            }
            CaveType::Big => {
                sum += count_to_end_2(input, next.to_string(), visited_small_caves, visited_twice);
            }
            _ => { panic!("Should never go back to start"); }
        }
    }
    sum
}

fn part1(input: &Input1) -> String {
    count_to_end_1(input, "start".to_string(), &mut HashSet::new()).to_string()
}

fn part2(input: &Input2) -> String {
    count_to_end_2(input, "start".to_string(), &mut HashSet::new(), &None).to_string()
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = Input::new();

    for line in lines.iter() {
        let mut parts = line.split("-");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        match cave_type(from) {
            CaveType::Start => {
                input.add_edge(from, to);
            }
            CaveType::End => {
                input.add_edge(to, from);
            }
            _ => {
                match cave_type(to) {
                    CaveType::Start => {
                        input.add_edge(to, from);
                    }
                    CaveType::End => {
                        input.add_edge(from, to);
                    }
                    _ => {
                        input.add_edge(from, to);
                        input.add_edge(to, from);
                    }
                }
            }
        }
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
