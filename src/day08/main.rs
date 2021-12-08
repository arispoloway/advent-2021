use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Line {
    digits: Vec<String>,
    code: Vec<String>,
}
type Input = Vec<Line>;
type Input1 = Input;
type Input2 = Input;
const INPUT_FILE: &str = "inputs/08.txt";

fn part1(input: &Input1) -> String {
    let mut sum = 0;
    for line in input {
        for digit in line.code.iter() {
            if digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 {
                sum += 1;
            }
        }
    }
    format!("{}", sum)
}

fn find<'a>(digits: &'a Vec<String>, predicate: &dyn Fn(&String) -> bool) -> &'a String {
    for digit in digits.iter() {
        if predicate(digit) {
            return digit;
        }
    }
    panic!()
}

// small enough that linear doesn't matter
fn diff(a: &String, b: &String) -> Vec<char> {
    let mut diff = Vec::new();
    let b_chars = b.chars().collect::<Vec<char>>();
    for char in a.chars() {
        if !b_chars.contains(&char) {
            diff.push(char);
        }
    }
    diff
}

fn intersect(a: &String, b: &String) -> Vec<char> {
    let mut intersection = Vec::new();
    let b_chars = b.chars().collect::<Vec<char>>();
    for char in a.chars() {
        if b_chars.contains(&char) {
            intersection.push(char);
        }
    }
    intersection
}

fn count_segments(digits: &Vec<String>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for digit in digits.iter() {
        for char in digit.chars() {
            let entry = counts.entry(char).or_insert(0);
            *entry += 1;
        }
    }
    counts
}

/*
Constraint satisfaction solvers are too much work:

1 - the one with 2 segments
4 - the one with 4 segments
7 - the one with 3 segments
8 - the one with 7 segments
f - the one in all but 1 of the segments
2 - the one that doesn't have f
c - the segment in 1 that's not f
d - the one in 2 & 4 that isn't c
0 - has everything except d
6 - has everything except c
9 - the number with 6 segments that isn't 0 or 6
5 - 9 minus segment c
3 - the last one
*/
fn decipher_digits(digits: &Vec<String>) -> HashMap<&String, i32> {
    let mut map: HashMap<&String, i32> = HashMap::new();
    let segment_counts = count_segments(digits);

    let d1 = find(digits, &|x: &String| -> bool { x.len() == 2 });
    let d7 = find(digits, &|x: &String| -> bool { x.len() == 3 });
    let d4 = find(digits, &|x: &String| -> bool { x.len() == 4 });
    let d8 = find(digits, &|x: &String| -> bool { x.len() == 7 });
    let f = *segment_counts
        .iter()
        .find_map(|(k, v)| if *v == 9 { Some(k) } else { None })
        .unwrap();
    let d2 = find(digits, &|x: &String| -> bool { !x.chars().any(|x| x == f) });
    let c = d1.chars().find(|x| *x != f).unwrap();
    let d = *intersect(d2, d4)
        .iter()
        .find_map(|v| if *v != c { Some(v) } else { None })
        .unwrap();
    let d0 = find(digits, &|x: &String| -> bool {
        x.len() == 6 && !x.chars().any(|x| x == d)
    });
    let d6 = find(digits, &|x: &String| -> bool {
        x.len() == 6 && !x.chars().any(|x| x == c)
    });
    let d9 = find(digits, &|x: &String| -> bool {
        x.len() == 6 && x != d0 && x != d6
    });
    let d5 = find(digits, &|x: &String| -> bool {
        x == &diff(d9, &c.to_string()).iter().collect::<String>()
    });
    let d3 = find(digits, &|x: &String| -> bool {
        x != d0
            && x != d1
            && x != d2
            && x != d4
            && x != d5
            && x != d6
            && x != d7
            && x != d8
            && x != d9
    });
    map.insert(d0, 0);
    map.insert(d1, 1);
    map.insert(d2, 2);
    map.insert(d3, 3);
    map.insert(d4, 4);
    map.insert(d5, 5);
    map.insert(d6, 6);
    map.insert(d7, 7);
    map.insert(d8, 8);
    map.insert(d9, 9);

    map
}

fn code(line: &Line) -> i32 {
    let mapping = decipher_digits(&line.digits);
    mapping[&line.code[0]] * 1000
        + mapping[&line.code[1]] * 100
        + mapping[&line.code[2]] * 10
        + mapping[&line.code[3]]
}

fn part2(input: &Input2) -> String {
    input.iter().map(|line| code(line)).sum::<i32>().to_string()
}

fn sort_digit(digit: String) -> String {
    let mut chars = digit.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.iter().collect::<String>()
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = Vec::new();
    for line in lines.iter() {
        let mut split = line.split(" | ");
        input.push(Line {
            digits: split
                .next()
                .unwrap()
                .split(" ")
                .map(|x| sort_digit(x.to_string()))
                .collect(),
            code: split
                .next()
                .unwrap()
                .split(" ")
                .map(|x| sort_digit(x.to_string()))
                .collect(),
        });
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
