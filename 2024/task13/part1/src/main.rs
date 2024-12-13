use std::fmt::Debug;
use std::{fs, ops};

const BUTTON_A_COST: u64 = 3;
const BUTTON_B_COST: u64 = 1;

const POSITION_ZERO: Position = Position { x: 0, y: 0 };

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: u64,
    y: u64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Score {
    a: u64,
    b: u64,
    value: u64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Game {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl ops::Mul<u64> for Position {
    type Output = Position;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let games: Vec<Game> = parse_lines(data);
    println!("games: {:?}", games);

    let sum = process_data(&games);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<Game> {
    data.split("\n\n")
        .map(|line| {
            let game_parts: Vec<Position> =
                line.split("\n").map(|item| parse_positions(item)).collect();
            // let (button_a, button_b, prize) = );
            return Game {
                button_a: game_parts[0],
                button_b: game_parts[1],
                prize: game_parts[2],
            };
        })
        .collect()
}

fn parse_positions(data: &str) -> Position {
    let (_, positions) = data.split_once(": ").unwrap();
    let (x_str, y_str) = positions.split_once(", ").unwrap();
    let x = parse_numbers(x_str);
    let y = parse_numbers(y_str);
    Position { x, y }
}

fn parse_numbers(data: &str) -> u64 {
    let (_, value) = data
        .split_once("+")
        .or_else(|| data.split_once("="))
        .unwrap();
    value.parse().unwrap()
}
fn process_data(games: &Vec<Game>) -> u64 {
    let mut sum: u64 = 0;
    for game in games {
        let &Game {
            prize,
            button_a,
            button_b,
        } = game;

        let mut score: Option<Score> = None;
        let max_b = max_button_count(&prize, &button_b);
        for b in (0..=max_b).rev() {
            let left_prize = prize - button_b * b;
            let max_a = max_button_count(&left_prize, &button_a);
            let final_prize = left_prize - button_a * max_a;
            if final_prize == POSITION_ZERO {
                let new_score = Score {
                    a: max_a,
                    b,
                    value: max_a * BUTTON_A_COST + b * BUTTON_B_COST,
                };
                // println!("prize equal for game={game:?} score={new_score:?}" );
                if score.is_none() || score.unwrap().value > new_score.value {
                    println!("new score for game={game:?} new_score={new_score:?}");
                    score = Some(new_score);
                }
            }
        }
        if let Some(score) = score {
            println!("final score for game={game:?} score={score:?}");
            sum += score.value;
        }
    }
    sum
}

fn max_button_count(prize: &Position, button: &Position) -> u64 {
    let max_x = prize.x / button.x;
    let max_y = prize.y / button.y;
    max_x.min(max_y)
}
