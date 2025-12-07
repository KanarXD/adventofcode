use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Guard {
    direction: Direction,
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

fn process_data(map: &Vec<Vec<PositionType>>, mut guard: Guard) -> u32 {
    let mut visited_map: Vec<Vec<bool>> = map
        .iter()
        .map(|positions| {
            return positions
                .into_iter()
                .map(|position| {
                    return match position {
                        PositionType::GuardPos(_) => true,
                        _ => false,
                    };
                })
                .collect();
        })
        .collect();

    loop {
        visited_map[guard.y][guard.x] = true;
        let out_map = match guard.direction {
            Direction::Up => move_guard(&mut guard, 0, -1, &mut visited_map, map, Direction::Right),
            Direction::Right => {
                move_guard(&mut guard, 1, 0, &mut visited_map, map, Direction::Down)
            }
            Direction::Down => move_guard(&mut guard, 0, 1, &mut visited_map, map, Direction::Left),
            Direction::Left => move_guard(&mut guard, -1, 0, &mut visited_map, map, Direction::Up),
        };
        if out_map {
            break;
        }
    }

    visited_map
        .iter()
        .map(|positions| positions.iter().filter(|&p| *p).count() as u32)
        .sum()
}

fn move_guard(
    guard: &mut Guard,
    add_x: i32,
    add_y: i32,
    visited_map: &mut Vec<Vec<bool>>,
    map: &Vec<Vec<PositionType>>,
    next_direction: Direction,
) -> bool {
    let try_y = guard.y as i32 + add_y;
    let try_x = guard.x as i32 + add_x;
    let width = visited_map[0].len() as i32;
    let height = visited_map.len() as i32;
    if try_x < 0 || try_x >= width || try_y < 0 || try_y >= height {
        return true;
    }
    let try_y = try_y as usize;
    let try_x = try_x as usize;
    if map[try_y][try_x] == PositionType::Blocked {
        guard.direction = next_direction
    } else {
        guard.y = try_y;
        guard.x = try_x;
        visited_map[guard.y][guard.x] = true;
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
                    guard = Some(Guard { direction, x, y });
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
