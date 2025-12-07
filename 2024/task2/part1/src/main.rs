use std::cmp::PartialEq;
use std::fs;

const MAX_INCREMENT: u32 = 3;
const MIN_INCREMENT: u32 = 1;

#[derive(Debug, PartialEq, Clone)]
enum State {
    Increment,
    Decrement,
    Broken,
}

fn main() {
    let file_path = "res/demo_input.txt";
    // let file_path = "res/input.txt";

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
        let mut previous_number = report[0];
        let difference: i32 = report[1] as i32 - previous_number as i32;
        let mut state = match difference {
            ..=-1 => State::Decrement,
            1.. => State::Increment,
            0 => continue,
        };
        for &number in report.iter().skip(1) {
            if !verify_numbers(state.clone(), previous_number, number) {
                state = State::Broken;
                break;
            }

            previous_number = number;
        }
        if state != State::Broken {
            sum += 1;
            println!("report={:?} good", report);
        }
    }

    sum
}

fn verify_numbers(direction: State, previous_number: u32, next_number: u32) -> bool {
    match direction {
        State::Increment => {
            if next_number <= previous_number {
                return false;
            }
        }
        State::Decrement => {
            if next_number >= previous_number {
                return false;
            }
        }
        State::Broken => return false,
    }
    let increment = previous_number.abs_diff(next_number);
    match increment {
        MIN_INCREMENT..=MAX_INCREMENT => true,
        _ => false,
    }
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
