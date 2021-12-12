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
        let mut input = Input {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        };

        for line in lines.iter() {
            let mut parts = line.split("-");
            let from = parts.next().unwrap().to_string();
            let to = parts.next().unwrap().to_string();

            // This needs to be done in two steps, because the compiler yells at me for borring
            // mutably twice.
            input.add_cave(from.clone());
            input.add_cave(to.clone());
            let cave_from = input.get_cave(from.clone());
            let cave_to = input.get_cave(to.clone());
            match cave_from {
                Cave::Start => {
                    input.add_edge(cave_from, cave_to);
                }
                Cave::End => {
                    input.add_edge(cave_to, cave_from);
                }
                _ => {
                    match cave_to {
                        Cave::Start => {
                            input.add_edge(cave_to, cave_from);
                        }
                        Cave::End => {
                            input.add_edge(cave_from, cave_to);
                        }
                        _ => {
                            input.add_edge(cave_from, cave_to);
                            input.add_edge(cave_to, cave_from);
                        }
                    }
                }
            }
        }
        input
    }

    fn add_cave(&mut self, cave_name: String) {
        self.nodes.get_or_insert(Cave::new(cave_name));
    }

    fn get_cave(&self, cave_name: String) -> &Cave {
        // a little annoying that this is how I need to look it up, but whatever
        self.nodes.get(&Cave::new(cave_name)).unwrap()
    }

    fn add_edge(&'a mut self, from: &'a Cave, to: &'a Cave) {
        self.edges.entry(from).or_insert(HashSet::new()).insert(to);
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
