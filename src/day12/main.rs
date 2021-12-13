#![feature(hash_set_entry)]

use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone)]
struct Input<'a> {
    edges: HashMap<&'a Cave, HashSet<&'a Cave>>,
    nodes: HashSet<Cave>,
}

type Input1<'a> = Input<'a>;
type Input2<'a> = Input<'a>;

const INPUT_FILE: &str = "inputs/12.txt";

impl<'a> Input<'a> {
    fn new(lines: &Vec<String>) -> Self {
        let mut nodes = HashSet::new();
        let mut edges: HashMap<&Cave, HashSet<&Cave>> = HashMap::new();

        for line in lines.iter() {
            let mut parts = line.split("-");
            let from = parts.next().unwrap().to_string();
            let to = parts.next().unwrap().to_string();

            let cave_from = nodes.get_or_insert(Cave::new(from.clone()));
            let cave_to = nodes.get_or_insert(Cave::new(to.clone()));
            match cave_from {
                Cave::Start => {
                    edges.entry(cave_from).or_insert(HashSet::new()).insert(cave_to);
                }
                Cave::End => {
                    edges.entry(cave_to).or_insert(HashSet::new()).insert(cave_from);
                }
                _ => {
                    match cave_to {
                        Cave::Start => {
                            edges.entry(cave_to).or_insert(HashSet::new()).insert(cave_from);
                        }
                        Cave::End => {
                            edges.entry(cave_from).or_insert(HashSet::new()).insert(cave_to);
                        }
                        _ => {
                            edges.entry(cave_from).or_insert(HashSet::new()).insert(cave_to);
                            edges.entry(cave_to).or_insert(HashSet::new()).insert(cave_from);
                        }
                    }
                }
            }
        }
        Input { edges, nodes }
    }

    fn get_edges(&'a self, from: &Cave) -> &'a HashSet<&Cave> {
        &self.edges[from]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Cave {
    Small(String),
    Big(String),
    Start,
    End,
}

impl Cave {
    fn new(name: String) -> Self {
        match name.as_str() {
            "start" => Cave::Start,
            "end" => Cave::End,
            x if x.to_uppercase() == x => Cave::Big(x.to_string()),
            x => Cave::Small(x.to_string()),
        }
    }
}

fn count_to_end<'a>(input: &'a Input, from: &'a Cave, visited_small_caves: &mut HashSet<&'a Cave>, visited_twice: &Option<&'a Cave>) -> usize {
    let mut sum = 0;
    for next in input.get_edges(from).iter() {
        match next {
            Cave::End => {
                sum += 1;
            }
            Cave::Small(_) => {
                if !visited_small_caves.contains(next) {
                    visited_small_caves.insert(next);
                    sum += count_to_end(input, next, visited_small_caves, visited_twice);
                    visited_small_caves.remove(next);
                } else if visited_twice.is_none() {
                    sum += count_to_end(input, next, visited_small_caves, &Some(next));
                }
            }
            Cave::Big(_) => {
                sum += count_to_end(input, next, visited_small_caves, visited_twice);
            }
            _ => { panic!("Should never go back to start"); }
        }
    }
    sum
}

fn part1(input: &Input1) -> String {
    count_to_end(input, &Cave::Start, &mut HashSet::new(), &Some(&Cave::Start)).to_string()
}

fn part2(input: &Input2) -> String {
    count_to_end(input, &Cave::End, &mut HashSet::new(), &None).to_string()
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
