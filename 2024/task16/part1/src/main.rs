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
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_axis(&self) -> Axis {
        match self {
            Direction::Up => Axis::Vertical,
            Direction::Down => Axis::Vertical,
            Direction::Left => Axis::Horizontal,
            Direction::Right => Axis::Horizontal,
        }
    }
    fn is_opposite(&self, other: &Self) -> bool {
        if self.get_axis() == other.get_axis() {
            return self != other;
        }
        false
    }
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
    let start_position = find_start_position(matrix);
    bfs(matrix, &start_position, &Direction::Right)
}

fn bfs(matrix: &Vec<Vec<Tile>>, start_position: &Position, start_direction: &Direction) -> u64 {
    let mut to_visit: LinkedList<(DirectedPosition, u64)> = LinkedList::new();
    to_visit.push_back((
        DirectedPosition {
            position: start_position.clone(),
            direction: start_direction.clone(),
        },
        0,
    ));
    let mut cache: HashMap<DirectedPosition, u64> = HashMap::new();
    let mut result = u64::MAX;
    while to_visit.len() > 0 {
        // println!("to_visit len: {}", to_visit.len());
        // println!("to_visit: {:?}", to_visit);
        let mut new_to_visit: LinkedList<(DirectedPosition, u64)> = LinkedList::new();
        for (directed_position, score) in to_visit.iter() {
            let DirectedPosition {
                position,
                direction,
            } = directed_position;
            if let Some(tile) = get_tile(matrix, &position) {
                if tile == Tile::Wall
                    || cache.get(directed_position).unwrap_or_else(|| &u64::MAX) < score
                {
                    continue;
                }
                cache.insert(*directed_position, *score);

                if tile == Tile::End {
                    if result > *score {
                        result = *score;
                    }
                    continue;
                }

                [
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                ]
                .into_iter()
                .filter(|checked_direction| !checked_direction.is_opposite(direction))
                .for_each(|checked_direction| {
                    let checked_position = direction_position(checked_direction, position);
                    let added_score = if checked_direction == *direction {
                        1
                    } else {
                        1001
                    };
                    let checked_score = score + added_score;
                    let checked_directed_position = DirectedPosition {
                        position: checked_position,
                        direction: checked_direction,
                    };
                    new_to_visit.push_back((checked_directed_position, checked_score));
                });
            }
        }
        to_visit = new_to_visit;
    }

    // println!("cache: {:?}", cache);

    result
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
