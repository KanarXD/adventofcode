use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

const ITERATIONS: usize = 75;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct NumberKey {
    number: u64,
    iteration: usize,
}

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
    let mut memory: HashMap<NumberKey, u64> = HashMap::new();

    let mut sum = 0;
    for &number in numbers {
        let number_key = NumberKey {
            number,
            iteration: 1,
        };
        sum += check_number(&mut memory, number_key);
    }

    sum
}

fn check_number(mut memory: &mut HashMap<NumberKey, u64>, mut number_key: NumberKey) -> u64 {
    if number_key.iteration > ITERATIONS {
        memory.insert(number_key, 1);
        return 1;
    }

    if let Some(number_sum) = memory.get(&number_key) {
        return *number_sum;
    }

    let number = number_key.number;

    if number == 0 {
        let new_number_key = NumberKey {
            number: 1,
            iteration: number_key.iteration + 1,
        };
        let number_sum = check_number(&mut memory, new_number_key);
        memory.insert(number_key, number_sum);
        return number_sum;
    }

    let number_string = number.to_string();
    if number_string.len() % 2 == 0 {
        let midpoint = number_string.len() / 2;
        let (left, right) = number_string.split_at(midpoint);

        let left_number = left.to_string().parse::<u64>().unwrap();
        let right_number = right.to_string().parse::<u64>().unwrap();

        let new_number_key_left = NumberKey {
            number: left_number,
            iteration: number_key.iteration + 1,
        };
        let number_sum_left = check_number(&mut memory, new_number_key_left);
        let new_number_key_right = NumberKey {
            number: right_number,
            iteration: number_key.iteration + 1,
        };
        let number_sum_right = check_number(&mut memory, new_number_key_right);
        let left_right_sum = number_sum_left + number_sum_right;
        memory.insert(number_key, left_right_sum);
        return left_right_sum;
    }

    let new_number_key = NumberKey {
        number: number_key.number * 2024,
        iteration: number_key.iteration + 1,
    };
    let number_sum = check_number(&mut memory, new_number_key);
    memory.insert(number_key, number_sum);
    number_sum
}
