use crate::Color::{BLUE, GREEN, RED};
use std::cmp::max;
use std::fs;

struct GameTurn {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Game {
    fn power(&self) -> u32 {
        self.max_red * self.max_green * self.max_blue
    }
}

enum Color {
    RED,
    GREEN,
    BLUE,
}
struct CubeMove {
    value: u32,
    color: Color,
}

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
    let games = parse_games(data);
    println!("{:?}", games);

    let game_powers: Vec<u32> = games.iter()
        .map(|game| game.power())
        .collect();

    game_powers
}

fn parse_games(data: String) -> Vec<Game> {
    let games: Vec<Game> = data.split("\n")
        .map(|line| {
            let mut parts = line.split(": ");
            let first_part = parts.next()
                .expect("line has to have first part");
            let second_part = parts.next()
                .expect("line has to have second part");
            return (first_part, second_part);
        })
        .map(|(first_part, second_part)| {
            let id: u32 = first_part.split(' ')
                .nth(1)
                .expect(format!("{first_part} has to have id").as_str())
                .parse()
                .expect(format!("{first_part} is not a number").as_str());
            parse_game(id, second_part)
        })
        .collect();
    games
}

fn parse_game(id: u32, games_string: &str) -> Game {
    let game_turns_string = games_string.split("; ");
    let mut game = Game { id, max_red: 0, max_green: 0, max_blue: 0 };
    for game_turn_string in game_turns_string {
        let GameTurn { red, green, blue } = parse_game_turn(game_turn_string);
        game.max_red = max(game.max_red, red);
        game.max_green = max(game.max_green, green);
        game.max_blue = max(game.max_blue, blue);
    }
    game
}

fn parse_game_turn(game_turn_string: &str) -> GameTurn {
    let game_moves_string = game_turn_string.split(", ");
    let mut game_turn = GameTurn { red: 0, green: 0, blue: 0 };
    for game_move_string in game_moves_string {
        let CubeMove { color, value } = parse_cube_move_string(game_move_string);
        match color {
            RED => { game_turn.red = value }
            GREEN => { game_turn.green = value }
            BLUE => { game_turn.blue = value }
        }
    }
    game_turn
}

fn parse_cube_move_string(cube_move_string: &str) -> CubeMove {
    let mut cube_move_parts = cube_move_string.split(' ');

    let value_string = cube_move_parts.next()
        .expect("cube_move_parts has to have value part");
    let value: u32 = value_string.parse()
        .expect(format!("{value_string} has to be a number").as_str());

    let color_string = cube_move_parts.next()
        .expect("cube_move_parts has to have color part");
    let color = match color_string {
        "red" => RED,
        "green" => GREEN,
        "blue" => BLUE,
        _ => { panic!("{color_string} has to be color") }
    };
    CubeMove { value, color }
}


