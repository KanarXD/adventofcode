use std::fmt::Debug;
use std::{fs, ops};

const WIDTH: isize = 101;
const HEIGHT: isize = 103;
// const WIDTH: isize = 11;
// const HEIGHT: isize = 7;

const MIDDLE_X: isize = WIDTH / 2;
const MIDDLE_Y: isize = HEIGHT / 2;

const ITERATIONS: usize = 100;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Robot {
    position: Position,
    velocity: Position,
}

impl ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let robots: Vec<Robot> = parse_lines(data);
    println!("robots: {:?}", robots);

    let sum = process_data(&robots);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<Robot> {
    data.split("\n")
        .map(|line| {
            let game_parts: Vec<Position> =
                line.split(" ").map(|item| parse_positions(item)).collect();
            return Robot {
                position: game_parts[0],
                velocity: game_parts[1],
            };
        })
        .collect()
}

fn parse_positions(data: &str) -> Position {
    let positions = &data[2..];
    let (x_str, y_str) = positions.split_once(",").unwrap();
    let x = x_str.parse().unwrap();
    let y = y_str.parse().unwrap();
    Position { x, y }
}

fn parse_numbers(data: &str) -> u64 {
    let (_, value) = data
        .split_once("+")
        .or_else(|| data.split_once("="))
        .unwrap();
    value.parse().unwrap()
}

fn verify_value(value: isize, max: isize) -> isize {
    if value >= max {
        return value - max;
    } else if value < 0 {
        return max + value;
    }
    value
}
fn process_data(robots: &Vec<Robot>) -> u64 {
    let mut result = generate_empty_matrix();

    let mut sum = 0;
    for robot in robots {
        let mut last_position = robot.position;
        for i in 0..ITERATIONS {
            last_position = last_position + robot.velocity;
            last_position.x = verify_value(last_position.x, WIDTH);
            last_position.y = verify_value(last_position.y, HEIGHT);
        }
        result[last_position.y as usize][last_position.x as usize] += 1;
    }
    // println!("result={:?}", result);
    print_matrix(&result);

    calculate_result_sum(&result)
}

fn print_matrix(matrix: &Vec<Vec<u64>>) {
    for row in matrix {
        let row_string = row
            .iter()
            .map(|p| {
                if *p == 0 {
                    return ".".to_string();
                }
                return p.to_string();
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{}", row_string);
    }
}

fn calculate_result_sum(result: &Vec<Vec<u64>>) -> u64 {
    let mut left_up = 0;
    let mut left_down = 0;
    let mut right_up = 0;
    let mut right_down = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let count = result[y as usize][x as usize];
            if x > MIDDLE_X && y > MIDDLE_Y {
                right_down += count;
            } else if x < MIDDLE_X && y < MIDDLE_Y {
                left_up += count;
            } else if x > MIDDLE_X && y < MIDDLE_Y {
                right_up += count;
            } else if x < MIDDLE_X && y > MIDDLE_Y {
                left_down += count;
            }
        }
    }

    left_up * left_down * right_down * right_up
}

fn generate_empty_matrix() -> Vec<Vec<u64>> {
    let mut result: Vec<Vec<u64>> = Vec::new();
    for y in 0..HEIGHT {
        let mut row: Vec<u64> = Vec::new();
        for x in 0..WIDTH {
            row.push(0);
        }
        result.push(row);
    }
    result
}
