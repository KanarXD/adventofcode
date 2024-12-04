use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
struct X {
    up_left: char,
    up_right: char,
    down_left: char,
    down_right: char,
    center: char,
}

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let reports: Vec<X> = parse_lines(data);
    println!("{:?}", reports.iter().count());

    let sum: u32 = process_data(reports);
    println!("sum={}", sum)
}

fn process_data(xs: Vec<X>) -> u32 {
    let mut count: u32 = 0;

    for x in xs {
        if x.center != 'A' {
            continue;
        }

        let diagonal_a = format!("{}{}{}", x.up_left, x.center, x.down_right);
        let diagonal_b = format!("{}{}{}", x.up_right, x.center, x.down_left);

        if !check_diagonal(&diagonal_a) {
            continue;
        }
        if !check_diagonal(&diagonal_b) {
            continue;
        }

        count += 1;
    }
    count
}

fn check_diagonal(diagonal: &str) -> bool {
    let mas_regex = Regex::new(r"MAS").unwrap();
    let sam_regex = Regex::new(r"SAM").unwrap();
    if mas_regex.is_match(&diagonal) {
        return true;
    }
    if sam_regex.is_match(&diagonal) {
        return true;
    }
    false
}

fn parse_lines(data: String) -> Vec<X> {
    let matrix: Vec<Vec<char>> = data
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    // println!("{:?}", matrix);
    let mut xs: Vec<X> = vec![];
    let height = matrix.len();
    let width = matrix[0].len();

    for y in 0..height {
        if y + 2 >= height {
            break;
        }
        for x in 0..width {
            if x + 2 >= width {
                break;
            }
            xs.push(X {
                up_left: matrix[x][y],
                up_right: matrix[x + 2][y],
                down_left: matrix[x][y + 2],
                down_right: matrix[x + 2][y + 2],
                center: matrix[x + 1][y + 1],
            })
        }
    }

    xs
}
