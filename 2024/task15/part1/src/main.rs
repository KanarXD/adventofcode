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
    Box,
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
    print_matrix(&tiles);
    println!("moves: {:?}", moves);
    println!("robot_position: {:?}", robot_position);

    let sum = process_data(&tiles, &moves, &robot_position);
    println!("sum={}", sum)
}

fn process_data(tiles: &Vec<Vec<Tile>>, moves: &Vec<Move>, robot_position: &Position) -> u64 {
    let mut tiles = tiles.clone();
    let mut robot_position = robot_position.clone();
    for robot_move in moves {
        // println!("move={:?}", robot_move);
        if let Some(new_robot_position) = try_move(&mut tiles, &robot_position, robot_move) {
            robot_position = new_robot_position;
        }
        // print_matrix(&tiles);
    }

    let score = calculate_score(&tiles);

    set_position(&mut tiles, &robot_position, Tile::Robot);
    print_matrix(&tiles);

    score
}

fn try_move(
    mut tiles: &mut Vec<Vec<Tile>>,
    actual_position: &Position,
    movement: &Move,
) -> Option<Position> {
    let actual_tile = get_tile(tiles, actual_position).unwrap();
    if actual_tile == Tile::Wall {
        return None;
    }

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
    if check_position(tiles, checked_position)
        || try_move(&mut tiles, &checked_position, movement).is_some()
    {
        let current_tile = get_tile(tiles, &actual_position).unwrap();
        set_position(&mut tiles, &actual_position, Tile::Empty);
        set_position(&mut tiles, &checked_position, current_tile);
        return Some(checked_position);
    }
    None
}

fn calculate_score(tiles: &Vec<Vec<Tile>>) -> u64 {
    let width = tiles[0].len();
    let height = tiles.len();
    let mut score = 0;
    for y in 0..height {
        for x in 0..width {
            if tiles[y][x] == Tile::Box {
                score += x + y * 100;
            }
        }
    }
    score as u64
}

fn check_position(tiles: &Vec<Vec<Tile>>, checked_position: Position) -> bool {
    let tile = get_tile(tiles, &checked_position);
    match tile {
        Some(Tile::Empty) => true,
        _ => false,
    }
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
                'O' => Tile::Box,
                '.' => Tile::Empty,
                '@' => {
                    robot_position = Some(Position { x, y });
                    Tile::Empty
                }
                _ => panic!("bad tile"),
            };
            row.push(tile);
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

fn print_matrix(matrix: &Vec<Vec<Tile>>) {
    for row in matrix {
        let row_string = row
            .iter()
            .map(|p| {
                return match *p {
                    Tile::Empty => ".",
                    Tile::Robot => "@",
                    Tile::Wall => "#",
                    Tile::Box => "O",
                }
                .to_string();
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{}", row_string);
    }
}
