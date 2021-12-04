use std::fs;

type Input = Vec<Vec<bool>>;
type Input1 = Input;
type Input2 = Input;
const INPUT_FILE: &str = "inputs/03.txt";

fn parse_input_1(lines: &Vec<String>) -> Input1 {
    parse_input(lines)
}

fn parse_input_2(lines: &Vec<String>) -> Input2 {
    parse_input(lines)
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut words: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        let mut word: Vec<bool> = Vec::new();
        for char in line.as_bytes() {
            word.push(*char == b'1');
        }
        words.push(word);
    }
    words
}

fn max_in_position(input: &Input, position: usize) -> Option<bool> {
    let mut true_count = 0;
    for row in input {
        if row[position] {
            true_count += 1;
        }
    }
    if true_count * 2 == input.len() {
        None
    } else {
        Some(true_count > input.len() / 2)
    }
}

fn invert_word(word: &Vec<bool>) -> Vec<bool> {
    let mut inverted: Vec<bool> = Vec::new();
    for bit in word {
        inverted.push(!*bit);
    }
    inverted
}

fn word_to_int(word: &Vec<bool>) -> u32 {
    let mut result = 0;
    let len = word.len();
    for (i, bit) in word.iter().enumerate() {
        if *bit {
            result += 2_u32.pow((len - i - 1) as u32);
        }
    }
    result
}

fn part1(input: &Input1) -> String {
    let len = input[0].len();
    let mut gamma: Vec<bool> = Vec::new();
    for i in 0..len {
        let max = max_in_position(input, i).expect("No max found");
        gamma.push(max);
    }
    let epsilon = invert_word(&gamma);
    format!("{:?}", word_to_int(&gamma) * word_to_int(&epsilon))
}

fn keep_bit(input: &mut Input, bit: usize, value: bool) {
    input.retain(|row| row[bit] == value);
}

fn rating(input: &Input, max: bool) -> u32 {
    let mut input = input.clone();
    let mut bit: usize = 0;

    while input.len() > 1 {
        match max_in_position(&input, bit) {
            Some(false) => keep_bit(&mut input, bit, !max),
            None | Some(true) => keep_bit(&mut input, bit, max),
        }
        bit += 1;
    }
    word_to_int(&input[0])
}

fn part2(input: &Input2) -> String {
    format!("{}", rating(input, true) * rating(input, false))
}

fn lines(filename: &str) -> Vec<String> {
    let file_string = fs::read_to_string(filename).expect("Something went wrong reading the file");
    file_string
        .split("\n")
        .map(|x| x.trim_end().to_string())
        .collect()
}

fn main() {
    let lines = lines(INPUT_FILE);

    println!("{}", part1(&parse_input_1(&lines)));
    println!("{}", part2(&parse_input_2(&lines)));
}
