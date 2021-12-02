use std::fs;

fn main() {
    let mut int_lines: Vec<i32> = Vec::new();
    for line in lines("inputs/01.txt") {
        int_lines.push(line.parse().unwrap());
    }

    let mut count = -1;
    let mut last = 0;

    for line in &int_lines {
        if line > &last {
            count += 1;
        }
        last = *line;
    }

    println!("{}", count);

    let mut count = -1;
    let mut last = 0;

    for index in 0..(int_lines.len() - 2) {
        let sum = int_lines[index..index + 3].iter().sum();
        if sum > last {
            count += 1;
        }
        last = sum;
    }
    println!("{}", count);
}

fn lines(filename: &str) -> Vec<String> {
    let file_string = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    file_string
        .split("\n")
        .map(|x| x.trim_end().to_string())
        .collect()
}
