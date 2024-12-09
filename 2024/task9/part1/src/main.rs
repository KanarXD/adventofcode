use std::fmt::Debug;
use std::fs;
// 5886536854
fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let locations: Vec<char> = parse_lines(data);
    println!("result: {:?}", locations.iter().collect::<String>());

    let sum = process_data(&locations);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<char> {
    let mut files = true;
    let mut result: Vec<char> = Vec::new();
    let mut index = 0;
    for letter in data.chars() {
        let count: u32 = letter.to_digit(10).unwrap();
        if files {
            files = false;
            for _ in 0..count {
                let index_char = index.to_string().chars().nth(0).unwrap();
                result.push(index_char);
            }
            index += 1;
        } else {
            files = true;
            for _ in 0..count {
                result.push('.');
            }
        }
    }
    result
}

fn process_data(locations: &Vec<char>) -> u64 {
    let mut result = locations.clone();
    loop {
        // println!("result={:?}", result);
        if let Some(start_index) = result.iter().position(|&c| c == '.') {
            // println!("start_index={}", start_index);
            if let Some(end_index) = result.iter().rposition(|&x| x != '.') {
                if end_index <= start_index {
                    break;
                }
                // println!("end_index={}", end_index);
                result[start_index] = locations[end_index];
                result[end_index] = '.';
            }
        }
    }

    println!("result: {:?}", result.iter().collect::<String>());

    result
        .iter()
        .filter(|&x| *x != '.')
        .map(|&c| c.to_digit(10))
        .flatten()
        .enumerate()
        .map(|(index, character)| index as u64 * character as u64)
        .sum()
}
