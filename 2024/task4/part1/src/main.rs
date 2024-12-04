use diagonal::{diagonal_pos_neg, diagonal_pos_pos};
use regex::{Match, Regex};
use std::fs;

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let reports: Vec<String> = parse_lines(data);
    println!("{:?}", reports);

    let sum: u32 = process_data(reports);
    println!("sum={}", sum)
}

fn process_data(lines: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for line in lines {
        let regex = Regex::new(r"XMAS").unwrap();
        let regex_reverse = Regex::new(r"SAMX").unwrap();
        count += find_in_line(line.as_str(), &regex);
        count += find_in_line(line.as_str(), &regex_reverse);
    }
    count
}

fn find_in_line(line: &str, regex: &Regex) -> u32 {
    let found: Vec<Match> = regex.find_iter(&line).collect();
    if found.len() > 0 {
        println!("{:?}", found);
    }
    found.len() as u32
}

fn parse_lines(data: String) -> Vec<String> {
    let matrix: Vec<Vec<char>> = data
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    println!("{:?}", matrix);
    let mut lines: Vec<String> = vec![];
    let height = matrix.len();
    let width = matrix[0].len();

    for line in &matrix {
        lines.push(line.iter().collect());
    }

    for x in 0..width {
        let mut vertical_line = String::new();
        for y in 0..height {
            vertical_line.push(matrix[y][x]);
        }
        lines.push(vertical_line);
    }

    diagonal_pos_pos(&matrix)
        .into_iter()
        .map(|diagonal| diagonal.into_iter().collect())
        .for_each(|diagonal| lines.push(diagonal));

    diagonal_pos_neg(&matrix)
        .into_iter()
        .map(|diagonal| diagonal.into_iter().collect())
        .for_each(|diagonal| lines.push(diagonal));

    lines
}
