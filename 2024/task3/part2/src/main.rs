use regex::Regex;
use std::cmp::PartialEq;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    Do,
    Dont,
    Mul(u32, u32),
}

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let reports: Vec<Action> = parse_numbers(data);
    println!("{:?}", reports);

    let sum: u32 = process_data(reports);
    println!("sum={}", sum)
}

fn process_data(actions: Vec<Action>) -> u32 {
    let mut add = true;
    let mut sum = 0;
    for action in actions {
        match action {
            Action::Do => {
                add = true;
            }
            Action::Dont => {
                add = false;
            }
            Action::Mul(left, right) => {
                if add {
                    sum += left * right;
                }
            }
        }
    }
    sum
}

fn parse_numbers(data: String) -> Vec<Action> {
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    regex
        .captures_iter(data.as_str())
        .map(|captures| {
            let value = captures.get(0).expect("group 0 has to exist").as_str();
            return match value {
                "do()" => Action::Do,
                "don't()" => Action::Dont,
                _ => {
                    let (i, [left, right]) = mul_regex
                        .captures(value)
                        .expect("value has to be mul(x,y)")
                        .extract();
                    let left = left.parse::<u32>().unwrap();
                    let right = right.parse::<u32>().unwrap();
                    Action::Mul(left, right)
                }
            };
        })
        .collect()
}
