#![feature(map_first_last)]

use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Input {
    string: String,
    counts: HashMap<(char, char), usize>,
    mapping: HashMap<(char, char), char>,
}

type Input1 = Input;
type Input2 = Input;

const INPUT_FILE: &str = "inputs/14.txt";

impl Input {
    fn new(lines: &Vec<String>) -> Self {
        let mut str_iter = lines.iter();
        let mut mapping = HashMap::new();
        let string = str_iter.next().unwrap().to_string();
        str_iter.next();
        for line in str_iter {
            let mut split = line.split(" -> ");
            let mut from = split.next().unwrap().chars();
            let to = split.next().unwrap().parse::<char>().unwrap();

            mapping.insert((from.next().unwrap(), from.next().unwrap()), to);
        }

        let mut counts = HashMap::new();
        let string_chars = string.chars().collect::<Vec<char>>();
        for i in 0..string_chars.len() - 1 {
            *counts.entry((string_chars[i], string_chars[i + 1])).or_insert(0) += 1;
        }

        Input { string, mapping, counts }
    }

    fn step(&mut self) {
        let mut new_counts: HashMap<(char, char), usize> = HashMap::new();

        for ((c1, c2), count) in self.counts.iter() {
            let mid = self.mapping[&(*c1, *c2)];
            *new_counts.entry((*c1, mid)).or_insert(0) += count;
            *new_counts.entry((mid, *c2)).or_insert(0) += count;
        }

        self.counts = new_counts;
    }

    fn score(&self) -> usize {
        let mut chars = self.string.chars();
        let first = chars.next().unwrap();
        let last = chars.last().unwrap();

        let mut counts: HashMap<char, usize> = HashMap::new();
        for (k, v) in self.counts.iter() {
            for c in [k.0, k.1].iter() {
                *counts.entry(*c).or_insert(0) += v;
            }
        }
        for (c, v) in counts.iter_mut() {
            *v /= 2;
            if *c == first || *c == last {
                *v += 1;
            }
        }

        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
}

fn part1(input: &Input1) -> String {
    let mut input = input.clone();
    for _ in 0..10 {
        input.step();
        input.score();
    }
    input.score().to_string()
}

fn part2(input: &Input2) -> String {
    let mut input = input.clone();
    for _ in 0..40 {
        input.step();
        input.score();
    }
    input.score().to_string()
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
