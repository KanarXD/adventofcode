use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    direction: Direction,
    position: Position,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn equals(&self, x: usize, y: usize) -> bool {
        self.x == x && self.y == y
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum PositionType {
    Empty,
    Blocked,
    GuardPos(Direction),
}

enum Status {
    Loop,
    OutOfMap,
}
fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let (map, guard): (Vec<Vec<PositionType>>, Guard) = parse_lines(data);
    println!("{:?}", map);
    println!("{:?}", guard);

    let sum: u32 = process_data(&map, guard);
    println!("sum={}", sum)
}

fn process_data(map: &Vec<Vec<PositionType>>, guard: Guard) -> u32 {
    let width = map[0].len();
    let height = map.len();
    println!("dimensions, width: {}, height: {}", width, height);
    let mut positions: HashSet<Position> = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            let checked_position = Position { x, y };
            if map[y][x] == PositionType::Blocked || guard.position.equals(x, y) {
                continue;
            }
            let mut cloned_map = map.clone();
            cloned_map[y][x] = PositionType::Blocked;
            println!("block x={x}, y={y}");
            let status = check_loops(&cloned_map, guard.clone());
            match status {
                Status::Loop => {
                    positions.insert(checked_position);
                }
                Status::OutOfMap => {
                    continue;
                }
            }
        }
    }
    println!("positions: {:?}", positions);
    positions.len() as u32
}

fn check_loops(map: &Vec<Vec<PositionType>>, mut guard: Guard) -> Status {
    let mut guards: HashSet<Guard> = HashSet::new();
    guards.insert(guard.clone());
    loop {
        let out_map = move_guard(map, &mut guard);
        // println!("check_loops guard={:?}, positions={:?}", guard, positions);
        if out_map {
            return Status::OutOfMap;
        }
        if guards.contains(&guard) {
            return Status::Loop;
        }
        guards.insert(guard.clone());
    }
}

fn move_guard(map: &Vec<Vec<PositionType>>, mut guard: &mut Guard) -> bool {
    match guard.direction {
        Direction::Up => try_move_guard(&mut guard, 0, -1, map, Direction::Right),
        Direction::Right => try_move_guard(&mut guard, 1, 0, map, Direction::Down),
        Direction::Down => try_move_guard(&mut guard, 0, 1, map, Direction::Left),
        Direction::Left => try_move_guard(&mut guard, -1, 0, map, Direction::Up),
    }
}

fn try_move_guard(
    guard: &mut Guard,
    add_x: i32,
    add_y: i32,
    map: &Vec<Vec<PositionType>>,
    next_direction: Direction,
) -> bool {
    let try_y = guard.position.y as i32 + add_y;
    let try_x = guard.position.x as i32 + add_x;
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    if try_x < 0 || try_x >= width || try_y < 0 || try_y >= height {
        return true;
    }
    let try_y = try_y as usize;
    let try_x = try_x as usize;
    if map[try_y][try_x] == PositionType::Blocked {
        guard.direction = next_direction
    } else {
        guard.position.y = try_y;
        guard.position.x = try_x;
    }
    false
}

fn parse_lines(data: String) -> (Vec<Vec<PositionType>>, Guard) {
    let mut map: Vec<Vec<PositionType>> = data
        .split("\n")
        .map(|line| {
            line.split("")
                .map(|x| {
                    return match x {
                        "#" => PositionType::Blocked,
                        ">" => PositionType::GuardPos(Direction::Right),
                        "<" => PositionType::GuardPos(Direction::Left),
                        "^" => PositionType::GuardPos(Direction::Up),
                        "v" => PositionType::GuardPos(Direction::Down),
                        _ => PositionType::Empty,
                    };
                })
                .collect::<Vec<PositionType>>()
        })
        .collect();

    let mut guard = None;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                PositionType::GuardPos(direction) => {
                    let position = Position { x, y };
                    guard = Some(Guard {
                        direction,
                        position,
                    });
                    map[y][x] = PositionType::Empty;
                    break;
                }
                _ => {}
            }
        }
        if guard.is_some() {
            break;
        }
    }

    (map, guard.unwrap())
}
