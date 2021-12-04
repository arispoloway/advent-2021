use std::fs;

type Input1 = Input;
type Input2 = Input;
const INPUT_FILE: &str = "inputs/04.txt";

#[derive(Debug, Clone)]
struct BingoCard([[u32; 5]; 5]);

impl BingoCard {
    fn new(input: &Vec<&String>) -> Self {
        let mut card = [[0; 5]; 5];
        for (i, row) in input.iter().enumerate() {
            for (j, cell) in row.split_whitespace().enumerate() {
                card[i][j] = cell.parse().unwrap();
            }
        }
        BingoCard(card)
    }

    fn get(&self, row: usize, col: usize) -> u32 {
        self.0[row][col]
    }
}

struct MarkableInput<'a> {
    input: &'a Input1,
    marks: Vec<[[bool; 5]; 5]>,
    next_selection: usize,
    finished: Vec<bool>,
}

impl<'a> MarkableInput<'a> {
    fn new(input: &Input1) -> MarkableInput {
        MarkableInput {
            input,
            next_selection: 0,
            marks: vec![[[false; 5]; 5]; input.cards.len()],
            finished: vec![false; input.cards.len()],
        }
    }

    // returns whether a bingo was made, and if it was, the score of the card it was made on
    fn draw_next(&mut self) -> Option<u32> {
        let number = self.input.draws[self.next_selection];
        let mut result = None;
        for card_num in 0..self.input.cards.len() {
            if let Some((row, column)) = self.mark_card(card_num, number) {
                if self.check_bingo(card_num, row, column) && !self.finished[card_num] {
                    result = Some(self.score(card_num, number));
                    self.finished[card_num] = true;
                }
            }
        }
        self.next_selection += 1;
        result
    }

    fn has_next(&self) -> bool {
        self.finished.iter().filter(|x| !**x).count() != 0
    }

    fn score(&self, card_num: usize, number: u32) -> u32 {
        let mut score = 0;
        for row in 0..5 {
            for column in 0..5 {
                if !self.marks[card_num][row][column] {
                    score += self.input.cards[card_num].get(row, column);
                }
            }
        }
        score * number
    }

    fn mark_card(&mut self, card_num: usize, number: u32) -> Option<(usize, usize)> {
        let card = &self.input.cards[card_num];
        for i in 0..5 {
            for j in 0..5 {
                if card.get(i, j) == number {
                    self.marks[card_num][i][j] = true;
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn check_bingo(&self, card_num: usize, row: usize, column: usize) -> bool {
        let marks = &self.marks[card_num];
        if marks[row][0] && marks[row][1] && marks[row][2] && marks[row][3] && marks[row][4] {
            return true;
        }
        if marks[0][column]
            && marks[1][column]
            && marks[2][column]
            && marks[3][column]
            && marks[4][column]
        {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Input {
    cards: Vec<BingoCard>,
    draws: Vec<u32>,
}

fn part1(input: &Input1) -> String {
    let mut input = MarkableInput::new(input);
    loop {
        match input.draw_next() {
            Some(score) => return format!("{}", score),
            None => (),
        }
    }
}

fn part2(input: &Input2) -> String {
    let mut input = MarkableInput::new(input);
    let mut worst_card = None;
    loop {
        if !input.has_next() {
            break;
        }
        match input.draw_next() {
            Some(score) => worst_card = Some(score),
            None => (),
        }
    }
    format!("{}", worst_card.unwrap())
}

fn parse_input_1(lines: &Vec<String>) -> Input1 {
    parse_input(lines)
}

fn parse_input_2(lines: &Vec<String>) -> Input2 {
    parse_input(lines)
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut iter = lines.iter();
    let draws = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mut cards: Vec<BingoCard> = Vec::new();

    loop {
        match iter.next() {
            None => break,
            _ => {
                let card_lines = vec![
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                ];
                cards.push(BingoCard::new(&card_lines));
            }
        }
    }

    Input { draws, cards }
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
