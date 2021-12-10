use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Input(Vec<Vec<u8>>);

type Input1 = Input;
type Input2 = Input;

const INPUT_FILE: &str = "inputs/09.txt";

type Coord = (usize, usize);

impl<'a> Input {
    fn iter(&'a self) -> impl Iterator<Item=(Coord, u8)> + 'a {
        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut x = 0;
        let mut y = 0;
        std::iter::from_fn(move || {
            if x == cols {
                x = 0;
                y += 1;
            }
            if y == rows {
                return None;
            }
            let res = Some(((x, y), self.0[y][x]));
            x += 1;
            res
        })
    }

    // TODO: use a more efficient iterator
    fn neighbors(&self, (x, y): Coord) -> Vec<(Coord, u8)> {
        let mut res = Vec::new();
        let (x, y) = (x as i32, y as i32);
        for &(dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && ny >= 0 && ny < self.0.len() as i32 && nx < self.0[0].len() as i32 {
                res.push(((nx as usize, ny as usize), self.0[ny as usize][nx as usize]));
            }
        }
        res
    }

    fn is_local_min(&self, (x, y): Coord) -> bool {
        for &((nx, ny), _) in &self.neighbors((x, y)) {
            if self.0[ny][nx] <= self.0[y][x] {
                return false;
            }
        }
        true
    }

    fn basin_size(&self, coord: Coord) -> usize {
        let mut queue = vec![coord];
        let mut visited = HashSet::new();
        visited.insert(coord);

        while let Some(coord) = queue.pop() {
            for &((nx, ny), v) in &self.neighbors(coord) {
                if visited.contains(&(nx, ny)) || v == 9 {
                    continue;
                }
                visited.insert((nx, ny));
                queue.push((nx, ny));
            }
        }
        visited.len()
    }
}

fn part1(input: &Input1) -> String {
    let mut sum: u32 = 0;
    for (coord, c) in input.iter() {
        if input.is_local_min(coord) {
            sum += (c + 1) as u32;
        }
    }
    format!("{}", sum)
}

fn part2(input: &Input2) -> String {
    let mut sizes = Vec::new();

    for (coord, _) in input.iter() {
        if input.is_local_min(coord) {
            sizes.push(input.basin_size(coord))
        }
    }

    // probably could be a bit smarter here
    sizes.sort();
    sizes.reverse();
    sizes.iter().take(3).product::<usize>().to_string()
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = Vec::new();
    for line in lines.iter() {
        input.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }
    Input(input)
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
