use std::fmt::Debug;
use std::fs;

const ITERATIONS: usize = 25;
const MULTIPLIER: u64 = 2024;

#[derive(Debug, Clone)]
struct Number {
    string: String,
    value: u64,
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let locations: Vec<Number> = parse_lines(data);
    println!("result: {:?}", locations);

    let sum = process_data(&locations);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<Number> {
    data.split(" ")
        .map(|number| {
            let value = number.parse::<u64>().unwrap();
            let string = number.to_string();
            return Number { value, string };
        })
        .collect()
}

fn process_data(numbers: &Vec<Number>) -> u64 {
    let mut old_numbers = Vec::new();
    let mut new_numbers = numbers.clone();

    for i in 0..ITERATIONS {
        old_numbers = new_numbers;
        new_numbers = Vec::new();
        for number in old_numbers {
            if number.value == 0 {
                new_numbers.push(Number {
                    value: 1,
                    string: "1".to_string(),
                });
                continue;
            }
            if number.string.len() % 2 == 0 {
                let midpoint = number.string.len() / 2;
                let (left, right) = number.string.split_at(midpoint);

                let left_number = left.to_string().parse::<u64>().unwrap();
                let right_number = right.to_string().parse::<u64>().unwrap();

                new_numbers.push(Number {
                    string: left.to_string(),
                    value: left_number,
                });
                new_numbers.push(Number {
                    string: right.to_string(),
                    value: right_number,
                });
                continue;
            }

            let new_number_value = number.value * MULTIPLIER;
            let new_number_string = new_number_value.to_string();
            new_numbers.push(Number {
                string: new_number_string,
                value: new_number_value,
            });
        }
        println!("iteration={} done", i + 1);
        // println!("iteration={} is {:?}", i + 1, new_numbers);
    }

    new_numbers.len() as u64
}
