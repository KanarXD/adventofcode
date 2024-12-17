use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct DirectedPosition {
    position: Position,
    direction: Direction,
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let matrix: Vec<Vec<Tile>> = parse_lines(data);
    println!("matrix: ");
    // print_matrix(&matrix, &vec![]);

    let sum = process_data(&matrix);
    println!("sum={}", sum)
}

fn process_data(matrix: &Vec<Vec<Tile>>) -> u64 {
    let mut visited = HashSet::new();
    let mut cache = HashMap::new();
    let start_position = find_start_position(matrix);
    let (score, path) = search(
        matrix,
        &mut visited,
        &start_position,
        &Direction::Right,
        &mut cache,
    )
    .unwrap();
    let cache_key = (
        DirectedPosition {
            position: start_position,
            direction: Direction::Right,
        },
        Position {
            x: start_position.x + 1,
            y: start_position.y,
        },
    );
    // let x = cache.get(&cache_key).unwrap();
    // println!("Cache: {x:?}");

    print_matrix(&matrix, &path);

    score
}

fn search(
    matrix: &Vec<Vec<Tile>>,
    mut visited: &mut HashSet<Position>,
    position: &Position,
    direction: &Direction,
    mut cache: &mut HashMap<(DirectedPosition, Position), Option<(u64, Vec<Position>)>>,
) -> Option<(u64, Vec<Position>)> {
    if let Some(tile) = get_tile(matrix, &position) {
        if tile == Tile::End {
            return Some((0, vec![*position]));
        }
        if ![Tile::Empty, Tile::Start].contains(&tile)
        // || visited.contains(&position)
        {
            return None;
        }
        visited.insert(position.clone());

        let directed_position = DirectedPosition {
            direction: direction.clone(),
            position: position.clone(),
        };

        let best_score: Option<(u64, Vec<Position>)> = [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ]
        .map(|d| {
            let checked_position = direction_position(d, position);
            if visited.contains(&checked_position) {
                return None;
            }
            let cache_key = (directed_position, checked_position);
            if let Some(cache_value) = cache.get(&cache_key) {
                return cache_value.clone();
            }
            let score = search(matrix, &mut visited, &checked_position, &d, &mut cache);
            if let Some((score, result_positions)) = score {
                let add_score = direction_score_add(direction, &d);
                let final_score = score + add_score;
                cache.insert(cache_key, Some((final_score, result_positions.clone())));
                return Some((final_score, result_positions));
            } else {
                cache.insert(cache_key, None);
            }
            None
        })
        .into_iter()
        .flatten()
        .min_by(|(s1, p1), (s2, p2)| s1.cmp(&s2));

        visited.remove(&position);

        if let Some((score, mut result_positions)) = best_score {
            result_positions.push(*position);
            return Some((score + 1, result_positions));
        }
    }
    None
}

fn direction_score_add(current_direction: &Direction, new_direction: &Direction) -> u64 {
    let add_score = 1000;
    add_score
        * match (current_direction, new_direction) {
            (Direction::Up, Direction::Up) => 0,
            (Direction::Up, Direction::Left) => 1,
            (Direction::Up, Direction::Right) => 1,
            (Direction::Up, Direction::Down) => 2,
            (Direction::Down, Direction::Up) => 2,
            (Direction::Down, Direction::Left) => 1,
            (Direction::Down, Direction::Right) => 1,
            (Direction::Down, Direction::Down) => 0,
            (Direction::Left, Direction::Up) => 1,
            (Direction::Left, Direction::Left) => 0,
            (Direction::Left, Direction::Right) => 2,
            (Direction::Left, Direction::Down) => 1,
            (Direction::Right, Direction::Up) => 1,
            (Direction::Right, Direction::Left) => 2,
            (Direction::Right, Direction::Right) => 0,
            (Direction::Right, Direction::Down) => 1,
        }
}

fn direction_position(direction: Direction, position: &Position) -> Position {
    match direction {
        Direction::Up => Position {
            x: position.x,
            y: position.y - 1,
        },
        Direction::Down => Position {
            x: position.x,
            y: position.y + 1,
        },
        Direction::Left => Position {
            x: position.x - 1,
            y: position.y,
        },
        Direction::Right => Position {
            x: position.x + 1,
            y: position.y,
        },
    }
}

fn get_tile(tiles: &Vec<Vec<Tile>>, position: &Position) -> Option<Tile> {
    if let Some(line) = tiles.get(position.y as usize) {
        if let Some(tile) = line.get(position.x as usize) {
            return Some(*tile);
        }
    }
    None
}

fn find_start_position(matrix: &Vec<Vec<Tile>>) -> Position {
    let width = matrix[0].len();
    let height = matrix.len();
    for y in 0..height {
        for x in 0..width {
            if matrix[y][x] == Tile::Start {
                return Position {
                    x: x as isize,
                    y: y as isize,
                };
            }
        }
    }
    panic!("start_position not found");
}

fn parse_lines(data: String) -> Vec<Vec<Tile>> {
    data.split("\n")
        .map(|line| {
            line.chars()
                .map(|c| {
                    return match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        'S' => Tile::Start,
                        'E' => Tile::End,
                        _ => panic!("bad input"),
                    };
                })
                .collect()
        })
        .collect()
}

fn print_matrix(matrix: &Vec<Vec<Tile>>, path: &Vec<Position>) {
    let height = matrix.len();
    let width = matrix[0].len();

    for y in 0..height {
        for x in 0..width {
            if path.contains(&Position {
                x: x as isize,
                y: y as isize,
            }) {
                print!("O");
            } else {
                let s = match matrix[y][x] {
                    Tile::Empty => " ",
                    Tile::Wall => "#",
                    Tile::Start => "S",
                    Tile::End => "E",
                }
                .to_string();

                print!("{}", s);
            }
        }
        println!();
    }
    println!("path length: {}", path.len());

    // for row in matrix {
    //     let row_string = row
    //         .iter()
    //         .map(|p| {
    //             return match *p {
    //                 Tile::Empty => ".",
    //                 Tile::Wall => "#",
    //                 Tile::Start => "S",
    //                 Tile::End => "E",
    //             }
    //             .to_string();
    //         })
    //         .collect::<Vec<String>>()
    //         .join("");
    //     println!("{}", row_string);
    // }
}
