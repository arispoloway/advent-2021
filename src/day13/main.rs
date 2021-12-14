#![feature(map_first_last)]

use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Input {
    dots: HashSet<Coord>,
    folds: Vec<Fold>,
    fold_index: usize,
}

#[derive(Debug, Clone)]
struct Fold {
    axis: Axis,
    value: usize,
}

#[derive(Debug, Clone)]
enum Axis {
    X,
    Y,
}

type Coord = (usize, usize);
type Input1 = Input;
type Input2 = Input;

const INPUT_FILE: &str = "inputs/13.txt";

impl Input {
    fn new(lines: &Vec<String>) -> Self {
        let mut dots = HashSet::new();
        let mut folds = Vec::new();

        let mut lines_iter = lines.iter();

        while let Some(line) = lines_iter.next() {
            if line == "" {
                break;
            }
            let mut split = line.split(",");
            dots.insert((
                split.next().unwrap().trim().parse().unwrap(),
                split.next().unwrap().trim().parse().unwrap(),
            ));
        }

        while let Some(line) = lines_iter.next() {
            let mut split = line[11..].split("=");
            let axis = split.next().unwrap();
            let value: usize = split.next().unwrap().trim().parse().unwrap();

            folds.push(Fold {
                axis: match axis {
                    "x" => Axis::X,
                    "y" => Axis::Y,
                    _ => panic!("Unknown axis: {}", axis),
                },
                value,
            });
        }

        Input { dots, folds, fold_index: 0 }
    }

    // returns whether or not it folded
    fn fold(&mut self) -> bool {
        if self.fold_index >= self.folds.len() {
            return false;
        }
        let fold = &self.folds[self.fold_index];
        let mut new_dots = HashSet::new();

        for dot in &self.dots {
            let new_coord: (i32, i32) = match fold.axis {
                Axis::X => (if dot.0 < fold.value { dot.0 as i32 } else { 2 * fold.value as i32 - dot.0 as i32 }, dot.1 as i32),
                Axis::Y => (dot.0 as i32, if dot.1 < fold.value { dot.1 as i32 } else { 2 * fold.value as i32 - dot.1 as i32 }),
            };
            if new_coord.0 < 0 || new_coord.1 < 0 {
                continue;
            }
            new_dots.insert((new_coord.0 as usize, new_coord.1 as usize));
        }
        match fold.axis {
            Axis::X => { self.dots.retain(|dot| dot.0 < fold.value); }
            Axis::Y => { self.dots.retain(|dot| dot.1 < fold.value); }
        }
        self.dots.extend(new_dots);

        self.fold_index += 1;
        true
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut max_x = 0;
        let mut max_y = 0;
        for dot in &self.dots {
            if dot.0 > max_x {
                max_x = dot.0;
            }
            if dot.1 > max_y {
                max_y = dot.1;
            }
        }
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        std::fmt::Result::Ok(())
    }
}


fn part1(input: &Input1) -> String {
    let mut input = input.clone();
    input.fold();
    input.dots.len().to_string()
}

fn part2(input: &Input2) -> String {
    let mut input = input.clone();
    while input.fold() {}
    format!("{}", input)
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
