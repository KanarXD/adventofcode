mod matrix;

use crate::matrix::Matrix;
use std::fs;
use std::ops::{Index, IndexMut};

const NOT_CONFIRMATION_SYMBOLS: &str = "0123456789.";

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String = fs::read_to_string(file_path)
        .expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let numbers: Vec<u32> = process_data(data);
    let sum: u32 = numbers.iter().sum::<u32>();

    println!("{:?}", numbers);
    println!("sum={}", sum)
}

fn process_data(data: String) -> Vec<u32> {
    let matrix = parse_matrix(data);
    let mut confirmed_numbers: Vec<u32> = vec![];
    for y in 0..matrix.height {
        let mut digits: Vec<char> = vec![];
        let mut confirmed = false;
        let mut last_char_digit = false;
        for x in 0..matrix.width {
            let char = matrix[y][x];
            let y: i32 = y as i32;
            let x: i32 = x as i32;
            let char_is_numeric = char.is_numeric();
            if char_is_numeric {
                digits.push(char);
            }
            match (char_is_numeric, last_char_digit, confirmed) {
                (true, true, false) => {
                    if check_up_and_down(&matrix, y, x) {
                        confirmed = true
                    }
                }
                (true, false, false) => {
                    if check_up_and_down(&matrix, y, x) ||
                        check_up_and_down(&matrix, y, x - 1) ||
                        check_point(&matrix, y, x - 1,
                        ) {
                        confirmed = true
                    }
                }
                (false, true, true) => {
                    let number = chars_to_number(&digits);
                    confirmed_numbers.push(number);
                    digits.clear();
                    confirmed = false;
                }
                (false, true, false) => {
                    if check_up_and_down(&matrix, y, x) || check_point(&matrix, y, x) {
                        let number = chars_to_number(&digits);
                        confirmed_numbers.push(number);
                    }
                    digits.clear();
                }
                _ => {}
            }
            last_char_digit = char_is_numeric;
        }
        if confirmed {
            let number = chars_to_number(&digits);
            confirmed_numbers.push(number);
        }
    }
    confirmed_numbers
}

fn check_up_and_down(matrix: &Matrix, y: i32, x: i32) -> bool {
    check_point(matrix, y - 1, x) ||
        check_point(matrix, y + 1, x)
}

fn check_point(matrix: &Matrix, y: i32, x: i32) -> bool {
    if matrix.contains(y, x) {
        let y: usize = y as usize;
        let x: usize = x
            as usize;
        return !NOT_CONFIRMATION_SYMBOLS.contains(matrix[y][x]);
    }
    false
}


fn chars_to_number(digits: &Vec<char>) -> u32 {
    digits.iter()
        .collect::<String>()
        .parse::<u32>()
        .expect(format!("{:?} are not a number", digits).as_str())
}

fn parse_matrix(data: String) -> Matrix {
    let lines: Vec<Vec<char>> = data.split('\n')
        .map(|line| line.chars().collect())
        .collect();
    Matrix::new(lines)
}