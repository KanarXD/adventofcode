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
    let mut file_offset = 0;
    loop {
        if let Some((file_start, file_end, number, file_size)) =
            find_file_positions(&result, file_offset)
        {
            // println!("file_offset={file_offset} file_start={file_start} file_end={file_end} number={number} file_size={file_size}");
            file_offset = locations.len() - file_start;

            let mut empty_offset = 0;
            loop {
                if let Some((empty_start, empty_end, empty_size)) =
                    find_empty_positions(&result, empty_offset)
                {
                    // println!("empty_offset={empty_offset} empty_start={empty_start} empty_end={empty_end} empty_size={empty_size}");
                    empty_offset = empty_end + 1;
                    if empty_start > file_end {
                        break;
                    }
                    if file_size > empty_size {
                        continue;
                    }

                    for i in 0..file_size {
                        result[i + empty_start] = result[i + file_start];
                        result[i + file_start] = None;
                    }
                    // println!("result: {:?}", result);

                    break;
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }

    // println!("result: {:?}", result);

    result
        .iter()
        .enumerate()
        .map(|(index, character)| {
            return if let (Some(number)) = character {
                index as u64 * number
            } else {
                0
            };
        })
        .sum()
}
fn find_empty_positions(
    locations: &Vec<Option<u64>>,
    offset: usize,
) -> Option<(usize, usize, usize)> {
    if let Some(start) = locations.iter().skip(offset).position(|&x| x.is_none()) {
        let new_start = offset + start;
        let end = locations
            .iter()
            .skip(new_start)
            .position(|&x| x.is_some())
            .unwrap_or_else(|| locations.len() - 1);

        let new_end = new_start + end - 1;
        let size = end;
        return Some((new_start, new_end, size));
    }
    None
}
fn find_file_positions(
    locations: &Vec<Option<u64>>,
    offset: usize,
) -> Option<(usize, usize, u64, usize)> {
    let start = locations
        .iter()
        .rev()
        .skip(offset)
        .position(|&x| x.is_some())
        .unwrap();
    let new_start = start + offset;
    let number = locations
        .iter()
        .rev()
        .skip(new_start)
        .next()
        .unwrap()
        .unwrap();
    if let Some(end) = locations
        .iter()
        .rev()
        .skip(new_start)
        .position(|&x| x != Some(number))
    {
        let size = end;
        let new_end = new_start + end - 1;
        let reversed_start = locations.len() - new_start - 1;
        let reversed_end = locations.len() - new_end - 1;
        return Some((reversed_end, reversed_start, number, size));
    }
    None
}
