use crate::Direction::{Decrement, Increment};
use std::cmp::PartialEq;
use std::fs;

const MAX_INCREMENT: u32 = 3;
const MIN_INCREMENT: u32 = 1;

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Good,
    Broken,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Increment,
    Decrement,
}

fn main() {
    // let file_path = "res/custom_input.txt";
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let reports: Vec<Vec<u32>> = parse_numbers(data);
    println!("{:?}", reports);

    let sum: u32 = process_data(reports);
    println!("sum={}", sum)
}

fn process_data(reports: Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for report in reports {
        if verify_report(&report) {
            sum += 1;
            println!("report={:?} good", report);
        }
    }

    sum
}

fn verify_report(report: &Vec<u32>) -> bool {
    let mut good = verify_report_direction(&report, Increment, true);
    if good {
        return true;
    }
    let mut good = verify_report_direction(&report, Decrement, true);
    if good {
        return true;
    }
    let report_skip_one = &report.iter().skip(1).copied().collect();
    let mut good = verify_report_direction(&report_skip_one, Increment, false);
    if good {
        return true;
    }
    let mut good = verify_report_direction(&report_skip_one, Decrement, false);
    if good {
        return true;
    }
    return false;
}

fn verify_numbers(direction: Direction, previous_number: u32, next_number: u32) -> bool {
    match direction {
        Increment => {
            if next_number <= previous_number {
                return false;
            }
        }
        Decrement => {
            if next_number >= previous_number {
                return false;
            }
        }
    }
    let increment = previous_number.abs_diff(next_number);
    match increment {
        MIN_INCREMENT..=MAX_INCREMENT => true,
        _ => false,
    }
}

fn verify_report_direction(report: &Vec<u32>, direction: Direction, mut can_skip: bool) -> bool {
    // let mut can_skip = true;
    let mut previous_number = report[0];

    for &next_number in report.iter().skip(1) {
        if verify_numbers(direction, previous_number, next_number) {
            previous_number = next_number;
        } else {
            if can_skip {
                can_skip = false;
            } else {
                return false;
            }
        }
    }
    true
}

fn parse_numbers(data: String) -> Vec<Vec<u32>> {
    data.split('\n')
        .map(|line| {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|value| value.parse::<u32>().expect("line value has to be a number"))
                .collect();
            return numbers;
        })
        .collect()
}
