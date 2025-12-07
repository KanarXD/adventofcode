use regex::Regex;
use std::cmp::PartialEq;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Mul {
    left: u32,
    right: u32,
}

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let reports: Vec<Mul> = parse_numbers(data);
    println!("{:?}", reports);

    let sum: u32 = process_data(reports);
    println!("sum={}", sum)
}

fn process_data(muls: Vec<Mul>) -> u32 {
    muls.into_iter().map(|m| m.left * m.right).sum()
}

fn parse_numbers(data: String) -> Vec<Mul> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    regex
        .captures_iter(data.as_str())
        .map(|c| c.extract())
        .map(|(i, [left, right])| {
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();
            return (left, right);
        })
        .map(|(left, right)| Mul { left, right })
        .collect()
}
