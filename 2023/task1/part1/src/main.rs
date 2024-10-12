use std::fs;

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String = fs::read_to_string(file_path)
        .expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let numbers = process_data(data);
    let sum = numbers.iter().sum::<u32>();

    println!("{:?}", numbers);
    println!("sum={}", sum)
}

fn process_data(data: String) -> Vec<u32> {
    data.split('\n')
        .map(extract_number)
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

fn extract_number(line: &str) -> String {
    let digits = extract_digits(line);
    let first = digits.first()
        .expect("line has to have a first number");
    let last = digits.last()
        .expect("line has to have a last number");
    let slice = [*first, *last];
    slice.iter().collect::<String>()
}

fn extract_digits(line: &str) -> Vec<char> {
    line.chars()
        .flat_map(|char| if char.is_numeric() { Some(char) } else { None })
        .collect::<Vec<char>>()
}

