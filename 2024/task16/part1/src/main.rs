use std::collections::{HashMap, LinkedList};
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Result {
    Visited,
    DeadEnd,
    Wall,
    Way(u64, LinkedList<Position>),
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
    let mut visited = LinkedList::new();
    let mut cache = HashMap::new();
    let start_position = find_start_position(matrix);
    if let Result::Way(score, path) = search(
        matrix,
        &mut visited,
        &start_position,
        &Direction::Right,
        &mut cache,
    ) {
        print_matrix(&matrix, &path);

        return score;
    }
    panic!("no way found");
}

fn search(
    matrix: &Vec<Vec<Tile>>,
    mut visited: &mut LinkedList<Position>,
    position: &Position,
    direction: &Direction,
    mut cache: &mut HashMap<(DirectedPosition, Position), Result>,
) -> Result {
    if let Some(tile) = get_tile(matrix, &position) {
        if tile == Tile::End {
            println!("found end");
            return Result::Way(0, LinkedList::new());
        }
        if tile == Tile::Wall {
            return Result::Wall;
        }
        if visited.contains(&position) {
            return Result::Visited;
        }

        visited.push_back(position.clone());
        // visited.insert(position.clone());

        let directed_position = DirectedPosition {
            direction: direction.clone(),
            position: position.clone(),
        };

        let neighbours: Vec<Result> = [
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::Up,
        ]
        .into_iter()
        .filter(|checked_direction| direction_score_add(direction, checked_direction) != 2000)
        .map(|checked_direction| {
            let checked_position = direction_position(checked_direction, position);
            let cache_key = (directed_position, checked_position);
            if let Some(cache_value) = cache.get(&cache_key) {
                return cache_value.clone();
            }
            let score = search(
                matrix,
                &mut visited,
                &checked_position,
                &checked_direction,
                &mut cache,
            );

            return match score {
                Result::Visited => Result::Visited,
                Result::Wall => {
                    cache.insert(cache_key, Result::Wall);
                    Result::Wall
                }
                Result::DeadEnd => {
                    cache.insert(cache_key, Result::DeadEnd);
                    Result::DeadEnd
                }
                Result::Way(score, result_positions) => {
                    let add_score = direction_score_add(direction, &checked_direction);
                    let final_score = score + add_score;
                    let result = Result::Way(final_score, result_positions.clone());
                    cache.insert(cache_key, result.clone());
                    result
                }
            };
        })
        .into_iter()
        .collect();

        let mut is_not_dead_end = false;

        let best_score = neighbours
            .into_iter()
            .map(|result| match result {
                Result::DeadEnd => None,
                Result::Visited => {
                    is_not_dead_end = true;
                    None
                }
                Result::Wall => None,
                Result::Way(score, path) => Some((score, path)),
            })
            .flatten()
            .min_by(|(s1, p1), (s2, p2)| s1.cmp(&s2));

        visited.pop_back();

        if let Some((score, mut result_positions)) = best_score {
            result_positions.push_back(*position);
            return Result::Way(score + 1, result_positions);
        } else if is_not_dead_end {
            return Result::Visited;
        } else {
            return Result::DeadEnd;
        }
    } else {
        return Result::Wall;
    }
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

fn print_matrix(matrix: &Vec<Vec<Tile>>, path: &LinkedList<Position>) {
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
