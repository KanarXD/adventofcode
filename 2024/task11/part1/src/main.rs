use std::fmt::Debug;
use std::fs;

const ITERATIONS: usize = 25;

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let locations: Vec<u64> = parse_lines(data);
    println!("result: {:?}", locations);

    let sum = process_data(&locations);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<u64> {
    data.split(" ")
        .map(|number| number.parse::<u64>().unwrap())
        .collect()
}

fn process_data(numbers: &Vec<u64>) -> u64 {
    let mut old_numbers = Vec::new();
    let mut new_numbers = numbers.clone();

    for i in 0..ITERATIONS {
        old_numbers = new_numbers;
        new_numbers = Vec::new();
        for number in old_numbers {
            if number == 0 {
                new_numbers.push(1);
                continue;
            }
            let number_string = number.to_string();
            if number_string.len() % 2 == 0 {
                let midpoint = number_string.len() / 2;
                let (left, right) = number_string.split_at(midpoint);

                let left_number = left.to_string().parse::<u64>().unwrap();
                let right_number = right.to_string().parse::<u64>().unwrap();

                new_numbers.push(left_number);
                new_numbers.push(right_number);
                continue;
            }
            new_numbers.push(number * 2024);
        }
        println!("iteration={} is {:?}", i + 1, new_numbers);
    }

    new_numbers.len() as u64
}
