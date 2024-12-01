use std::collections::HashMap;
use std::fs;

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let (left_numbers, right_numbers): (Vec<u32>, Vec<u32>) = parse_numbers(data);
    println!("{:?}", left_numbers);
    println!("{:?}", right_numbers);

    let sum: u32 = process_data(left_numbers, right_numbers);
    println!("sum={}", sum)
}

fn process_data(mut left_numbers: Vec<u32>, mut right_numbers: Vec<u32>) -> u32 {
    right_numbers.sort();

    let mut right_number_counts: HashMap<u32, u32> = HashMap::new();
    for right in right_numbers {
        let count = right_number_counts.entry(right).or_insert(0);
        *count += 1;
    }

    println!("{:?}", right_number_counts);
    let mut sum: u32 = 0;

    for left in left_numbers {
        let left_number_count = *right_number_counts.entry(left).or_insert(0);
        sum += left * left_number_count;
    }
    sum
}
fn parse_numbers(data: String) -> (Vec<u32>, Vec<u32>) {
    let mut left_numbers: Vec<u32> = vec![];
    let mut right_numbers: Vec<u32> = vec![];
    data.split('\n')
        .map(|line| {
            let (left, right) = line.split_once("   ").expect("line has to be split");
            let left = left.parse::<u32>().expect("Left should be an integer");
            let right = right.parse::<u32>().expect("Right should be an integer");
            return (left, right);
        })
        .for_each(|(left, right)| {
            left_numbers.push(left);
            right_numbers.push(right);
        });
    assert_eq!(left_numbers.len(), right_numbers.len());
    (left_numbers, right_numbers)
}
