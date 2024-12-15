use std::fmt::Debug;
use std::fs;

const ITERATIONS: usize = 100;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let (tiles, moves, robot_position): (Vec<Vec<Tile>>, Vec<Move>, Position) = parse_lines(data);
    println!("tiles: ");
    print_matrix(&tiles, &robot_position);
    // println!("moves: {:?}", moves);
    println!("robot_position: {:?}", robot_position);

    let sum = process_data(&tiles, &moves, &robot_position);
    println!("sum={}", sum)
}

fn process_data(tiles: &Vec<Vec<Tile>>, moves: &Vec<Move>, robot_position: &Position) -> u64 {
    let mut tiles = tiles.clone();
    let mut robot_position = robot_position.clone();
    for robot_move in moves {
        if let Some(new_robot_position) = try_move(&mut tiles, &robot_position, robot_move) {
            do_move(&mut tiles, &robot_position, robot_move);
            robot_position = new_robot_position;
            println!("move={:?}", robot_move);
            // print_matrix(&tiles, &robot_position);
        } else {
            println!("move={:?} failed", robot_move);
        }
    }

    print_matrix(&tiles, &robot_position);

    calculate_score(&tiles)
}

fn do_move_box(mut tiles: &mut Vec<Vec<Tile>>, actual_position_left: &Position, movement: &Move) {
    let actual_position_right = Position {
        x: actual_position_left.x + 1,
        y: actual_position_left.y,
    };

    let checked_position_left = get_move_position(&actual_position_left, movement);
    let checked_position_right = get_move_position(&actual_position_right, movement);

    do_move(&mut tiles, &checked_position_left, movement);
    do_move(&mut tiles, &checked_position_right, movement);

    set_position(&mut tiles, &checked_position_left, Tile::BoxLeft);
    set_position(&mut tiles, &checked_position_right, Tile::BoxRight);

    set_position(&mut tiles, &actual_position_left, Tile::Empty);
    set_position(&mut tiles, &actual_position_right, Tile::Empty);
}

fn do_move(mut tiles: &mut Vec<Vec<Tile>>, actual_position: &Position, movement: &Move) {
    let current_tile = get_tile(tiles, actual_position).unwrap();
    match current_tile {
        Tile::Empty => {}
        Tile::BoxLeft => {
            if [Move::Up, Move::Down].contains(&movement) {
                do_move_box(&mut tiles, actual_position, movement);
            } else {
                let checked_position = get_move_position(actual_position, movement);
                do_move(&mut tiles, &checked_position, movement);
                set_position(&mut tiles, &checked_position, current_tile);
                set_position(&mut tiles, &actual_position, Tile::Empty);
            }
        }
        Tile::BoxRight => {
            if [Move::Up, Move::Down].contains(&movement) {
                let actual_position_left = Position {
                    x: actual_position.x - 1,
                    y: actual_position.y,
                };
                do_move_box(&mut tiles, &actual_position_left, movement);
            } else {
                let checked_position = get_move_position(actual_position, movement);
                do_move(&mut tiles, &checked_position, movement);
                set_position(&mut tiles, &checked_position, current_tile);
                set_position(&mut tiles, &actual_position, Tile::Empty);
            }
        }
        Tile::Robot => {
            let checked_position = get_move_position(actual_position, movement);
            do_move(&mut tiles, &checked_position, movement);
            set_position(&mut tiles, &checked_position, Tile::Robot);
            set_position(&mut tiles, &actual_position, Tile::Empty);
        }
        Tile::Wall => {}
    }
}

fn try_move(
    mut tiles: &mut Vec<Vec<Tile>>,
    actual_position: &Position,
    movement: &Move,
) -> Option<Position> {
    let actual_tile = get_tile(tiles, actual_position).unwrap();
    match actual_tile {
        Tile::Empty => {
            return Some(actual_position.clone());
        }
        Tile::Wall => return None,
        Tile::BoxLeft => {
            if [Move::Up, Move::Down].contains(&movement) {
                let checked_position_left = get_move_position(actual_position, movement);
                let checked_position_right = Position {
                    x: checked_position_left.x + 1,
                    y: checked_position_left.y,
                };

                if try_move(&mut tiles, &checked_position_left, movement).is_some()
                    && try_move(&mut tiles, &checked_position_right, movement).is_some()
                {
                    return Some(checked_position_left);
                }
            } else {
                let checked_position = get_move_position(actual_position, movement);
                if try_move(&mut tiles, &checked_position, movement).is_some() {
                    return Some(checked_position);
                }
            }
        }
        Tile::BoxRight => {
            if [Move::Up, Move::Down].contains(&movement) {
                let checked_position_right = get_move_position(actual_position, movement);
                let checked_position_left = Position {
                    x: checked_position_right.x - 1,
                    y: checked_position_right.y,
                };
                if try_move(&mut tiles, &checked_position_left, movement).is_some()
                    && try_move(&mut tiles, &checked_position_right, movement).is_some()
                {
                    return Some(checked_position_right);
                }
            } else {
                let checked_position = get_move_position(actual_position, movement);
                if try_move(&mut tiles, &checked_position, movement).is_some() {
                    return Some(checked_position);
                }
            }
        }
        Tile::Robot => {
            let checked_position = get_move_position(actual_position, movement);
            if try_move(&mut tiles, &checked_position, movement).is_some() {
                return Some(checked_position);
            }
        }
    }
    None
}

fn get_move_position(actual_position: &Position, movement: &Move) -> Position {
    let checked_position: Position = match movement {
        Move::Left => Position {
            x: actual_position.x - 1,
            y: actual_position.y,
        },
        Move::Right => Position {
            x: actual_position.x + 1,
            y: actual_position.y,
        },
        Move::Up => Position {
            x: actual_position.x,
            y: actual_position.y - 1,
        },
        Move::Down => Position {
            x: actual_position.x,
            y: actual_position.y + 1,
        },
    };
    checked_position
}

fn check_position(tiles: &Vec<Vec<Tile>>, checked_position: Position) -> bool {
    let tile = get_tile(tiles, &checked_position);
    match tile {
        Some(Tile::Empty) => true,
        _ => false,
    }
}

fn calculate_score(tiles: &Vec<Vec<Tile>>) -> u64 {
    let width = tiles[0].len();
    let height = tiles.len();
    let mut score = 0;
    for y in 0..height {
        for x in 0..width {
            if tiles[y][x] == Tile::BoxLeft {
                score += x + y * 100;
            }
        }
    }
    score as u64
}

fn set_position(tiles: &mut Vec<Vec<Tile>>, position: &Position, tile: Tile) {
    tiles[position.y as usize][position.x as usize] = tile;
}

fn get_tile(tiles: &Vec<Vec<Tile>>, position: &Position) -> Option<Tile> {
    if let Some(line) = tiles.get(position.y as usize) {
        if let Some(tile) = line.get(position.x as usize) {
            return Some(*tile);
        }
    }
    None
}

fn parse_lines(data: String) -> (Vec<Vec<Tile>>, Vec<Move>, Position) {
    let (tiles_string, moves_string) = data.split_once("\n\n").unwrap();
    let tiles_char: Vec<Vec<char>> = tiles_string
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let width = tiles_char[0].len() as isize;
    let height = tiles_char.len() as isize;
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut robot_position: Option<Position> = None;

    for y in 0..height {
        let mut row: Vec<Tile> = Vec::new();
        for x in 0..width {
            let tile = match tiles_char[y as usize][x as usize] {
                '#' => Tile::Wall,
                'O' => Tile::BoxLeft,
                '.' => Tile::Empty,
                '@' => {
                    robot_position = Some(Position { x: x * 2, y });
                    Tile::Robot
                }
                _ => panic!("bad tile"),
            };

            row.push(tile);
            if tile == Tile::BoxLeft {
                row.push(Tile::BoxRight);
            } else if tile == Tile::Robot {
                row.push(Tile::Empty);
            } else {
                row.push(tile);
            }
        }
        tiles.push(row);
    }

    let moves: Vec<Move> = moves_string
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            '^' => Move::Up,
            'v' => Move::Down,
            _ => panic!("bad move"),
        })
        .collect();

    (tiles, moves, robot_position.unwrap())
}

fn print_matrix(matrix: &Vec<Vec<Tile>>, robot_position: &Position) {
    let mut matrix = matrix.clone();
    // set_position(&mut matrix, &robot_position, Tile::Robot);
    for row in matrix {
        let row_string = row
            .iter()
            .map(|p| {
                return match *p {
                    Tile::Empty => ".",
                    Tile::Robot => "@",
                    Tile::Wall => "#",
                    Tile::BoxLeft => "[",
                    Tile::BoxRight => "]",
                }
                .to_string();
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{}", row_string);
    }
}
