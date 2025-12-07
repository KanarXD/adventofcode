use std::fmt::Debug;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}
impl Operation {
    fn perform(&self, left: u64, right: u64) -> u64 {
        match self {
            Operation::Add => left + right,
            Operation::Multiply => left * right,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn calculate(&self) -> u64 {
        self.numbers[1..]
            .iter()
            .fold(self.numbers[0], |acc, number| {
                self.operation.perform(acc, *number)
            })
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let problems = parse_lines(data);
    println!("problems = {:?}", problems);

    let sum = process_data(&problems);
    println!("sum={}", sum)
}

fn process_data(problems: &Vec<Problem>) -> u64 {
    problems.iter().map(|problem| problem.calculate()).sum()
}

fn parse_lines(data: String) -> Vec<Problem> {
    let lines: Vec<&str> = data.lines().collect();

    let mut problems: Vec<Problem> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|operation| match operation {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => panic!("unknown operation"),
        })
        .map(|operation| Problem {
            numbers: vec![],
            operation,
        })
        .collect();

    for &line in lines[0..lines.len() - 1].iter() {
        line.split_whitespace()
            .enumerate()
            .for_each(|(index, number)| {
                let number = number.parse::<u64>().unwrap();
                problems[index].numbers.push(number);
            });
    }

    problems
}
