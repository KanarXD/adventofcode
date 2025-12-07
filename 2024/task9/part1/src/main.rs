use std::fmt::Debug;
use std::fs;

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let locations: Vec<Option<u64>> = parse_lines(data);
    println!("result: {:?}", locations);

    let sum = process_data(&locations);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<Option<u64>> {
    let mut files = true;
    let mut result: Vec<Option<u64>> = Vec::new();
    let mut index: u64 = 0;
    for letter in data.chars() {
        let count: u32 = letter.to_digit(10).unwrap();
        if files {
            files = false;
            for _ in 0..count {
                result.push(Some(index));
            }
            index += 1;
        } else {
            files = true;
            for _ in 0..count {
                result.push(None);
            }
        }
    }
    result
}

fn process_data(locations: &Vec<Option<u64>>) -> u64 {
    let mut result = locations.clone();
    loop {
        // println!("result={:?}", result);
        if let Some(start_index) = result.iter().position(|&c| c == None) {
            // println!("start_index={}", start_index);
            if let Some(end_index) = result.iter().rposition(|&x| x != None) {
                if end_index <= start_index {
                    break;
                }
                // println!("end_index={}", end_index);
                result[start_index] = locations[end_index];
                result[end_index] = None;
            }
        }
    }

    println!("result: {:?}", result);

    result
        .iter()
        .flatten()
        .enumerate()
        .map(|(index, character)| index as u64 * character)
        .sum()
}
