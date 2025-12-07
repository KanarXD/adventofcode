use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use std::fmt::Debug;
use std::{fs, ops};

const BUTTON_A_COST: u64 = 3;
const BUTTON_B_COST: u64 = 1;

const POSITION_ZERO: Position = Position { x: 0, y: 0 };

const PRIZE_MULTIPLIER: u64 = 10000000000000;

const MAX_ERROR: f64 = 0.0001;

// 0.1       98424029434594
// 0.01      95843948914827
// 0.001     95843948914827
// 0.0001    95404451197398
// 0.00001   17049071680535
// 0.000001  17049071680535
// 0.0000001 17049071680535

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

impl ops::Add<u64> for Position {
    type Output = Position;

    fn add(self, rhs: u64) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
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
            return Game {
                button_a: game_parts[0],
                button_b: game_parts[1],
                prize: game_parts[2] + PRIZE_MULTIPLIER,
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

        /*
        minimize 3a + b
        with
            xa + xb = prize_x
            ya + yb = prize_y
        */

        variables! {
            vars:
               a >= 0;
               b >= 0;
        } // variables can also be added dynamically
        let solution = vars
            .maximise(BUTTON_A_COST as f64 * a + BUTTON_B_COST as f64 * b)
            .using(default_solver) // multiple solvers available
            .with(constraint!(
                button_a.x as f64 * a + button_b.x as f64 * b == prize.x as f64
            ))
            .with(constraint!(
                button_a.y as f64 * a + button_b.y as f64 * b == prize.y as f64
            ))
            .solve();

        if let Ok(result) = solution {
            let a = result.value(a);
            let b = result.value(b);
            if exceeds_error(a) || exceeds_error(b) {
                continue;
            }
            let a_round = a.round() as u64;
            let b_round = b.round() as u64;

            let value = BUTTON_A_COST * a_round + BUTTON_B_COST * b_round;

            let score = Score {
                a: a_round,
                b: b_round,
                value,
            };
            println!("final score for game={game:?} score={score:?}");
            sum += value;
        }
    }
    sum
}

fn exceeds_error(number: f64) -> bool {
    let number_round = number.round();
    (number_round - number).abs() > MAX_ERROR
}
