use std::fs;

#[derive(Debug, Clone)]
struct Calculation {
    result: u64,
    numbers: Vec<u64>,
}

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let calculations: Vec<Calculation> = parse_lines(data);
    println!("{:?}", calculations);

    let sum: u64 = process_data(&calculations);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<Calculation> {
    data.split("\n")
        .map(|line| {
            let (result, numbers) = line.split_once(": ").unwrap();
            let result = result.parse::<u64>().expect("result should be a number");
            let numbers = numbers
                .split(" ")
                .map(|num| num.parse::<u64>().expect("numbers should be a numbers"))
                .collect::<Vec<u64>>();
            return Calculation { result, numbers };
        })
        .collect()
}

fn process_data(calculations: &Vec<Calculation>) -> u64 {
    let mut sum: u64 = 0;
    for calculation in calculations {
        if check_calculation(calculation.result, 0, &calculation.numbers, 0) {
            sum += calculation.result;
        }
    }
    sum
}

fn check_calculation(expected_result: u64, result: u64, numbers: &Vec<u64>, index: usize) -> bool {
    if index >= numbers.len() {
        return expected_result == result;
    }
    let number = &numbers[index];
    if check_calculation(expected_result, result + number, numbers, index + 1) {
        return true;
    } else if check_calculation(expected_result, result * number, numbers, index + 1) {
        return true;
    }
    false
}
