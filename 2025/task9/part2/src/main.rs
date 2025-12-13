use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Ord, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x >= other.x && self.y >= other.y {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Position {
    fn rect_size(&self, position: &Position) -> usize {
        (self.y.abs_diff(position.y) + 1) * (self.x.abs_diff(position.x) + 1)
    }

    fn corner_pos(&self, position: &Position) -> Position {
        Position {
            x: self.x,
            y: position.y,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Red,
    Green,
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let positions = parse_lines(data);
    println!("positions = {:?}", positions);

    let sum = process_data(positions);
    println!("sum={}", sum)
}

fn process_data(positions: Vec<Position>) -> usize {
    let matrix = &create_matrix(&positions);

    let mut rects: BTreeMap<usize, Vec<(Position, Position)>> = BTreeMap::new();

    for i in 0..positions.len() {
        'j_loop: for j in i + 1..positions.len() {
            let position_a = positions[i];
            let position_b = positions[j];
            let corner_pos_a = position_a.corner_pos(&position_b);
            let corner_pos_b = position_b.corner_pos(&position_a);

            if is_broken_line(matrix, &position_a, &corner_pos_a)
                || is_broken_line(matrix, &position_a, &corner_pos_b)
                || is_broken_line(matrix, &position_b, &corner_pos_a)
                || is_broken_line(matrix, &position_b, &corner_pos_b)
            {
                // println!(
                //     "broken rect for positions={:?} and {:?}",
                //     position_a, position_b
                // );
                continue 'j_loop;
            }

            let rect_size = position_a.rect_size(&position_b);
            rects
                .entry(rect_size)
                .or_insert(Vec::new())
                .push((position_a, position_b));
        }
    }

    println!("rects={:?}", rects);
    *rects.keys().last().unwrap()
}

fn is_broken_line(matrix: &Vec<Vec<Tile>>, p_a: &Position, p_b: &Position) -> bool {
    run_for_line(p_a, p_b, |y, x| {
        if y >= matrix.len() || x >= matrix[0].len() {
            return true;
        }
        matrix[y][x] == Tile::Empty
    })
}
fn create_matrix(positions: &Vec<Position>) -> Vec<Vec<Tile>> {
    let max_x = positions.iter().map(|p| p.x).max().unwrap();
    let max_y = positions.iter().map(|p| p.y).max().unwrap();

    let border = 0;

    let mut matrix: Vec<Vec<Tile>> = (0..=max_y + border)
        .map(|_| (0..=max_x + border).map(|_| Tile::Empty).collect())
        .collect();

    print_matrix(&matrix);

    positions.iter().for_each(|position| {
        matrix[position.y][position.x] = Tile::Red;
    });
    print_matrix(&matrix);

    for i in 0..positions.len() - 1 {
        let p_a = positions[i];
        let p_b = positions[i + 1];

        draw_green_line(&mut matrix, &p_a, &p_b);
    }
    draw_green_line(&mut matrix, &positions[0], &positions[positions.len() - 1]);

    print_matrix(&matrix);

    'y_loop: for y in 0..matrix.len() {
        let mut inside = false;
        let mut x = 0;
        'x_loop: while x < matrix[0].len() {
            let tile = &mut matrix[y][x];
            match tile {
                Tile::Empty => {
                    if inside {
                        match find_next_other_tile(&matrix[y], &vec![Tile::Empty], x) {
                            None => {
                                continue 'y_loop;
                            }
                            Some(nx) => {
                                for cx in x..nx {
                                    matrix[y][cx] = Tile::Green;
                                }
                                x = nx;
                                continue 'x_loop;
                            }
                        };
                    } else {
                        match find_next_other_tile(&matrix[y], &vec![Tile::Empty], x) {
                            None => {
                                continue 'y_loop;
                            }
                            Some(nx) => {
                                x = nx;
                                continue 'x_loop;
                            }
                        };
                    }
                }
                Tile::Red | Tile::Green => {
                    match find_next_other_tile(&matrix[y], &vec![Tile::Red, Tile::Green], x) {
                        None => {
                            continue 'y_loop;
                        }
                        Some(nx) => {
                            if nx != x + 2 {
                                inside = !inside;
                            }
                            x = nx;
                            continue 'x_loop;
                        }
                    }
                }
            }
        }
    }

    print_matrix(&matrix);
    matrix
}

fn find_next_other_tile(line: &Vec<Tile>, tiles: &Vec<Tile>, start_x: usize) -> Option<usize> {
    for x in start_x..line.len() {
        if !tiles.contains(&line[x]) {
            return Some(x);
        }
    }

    None
}

fn draw_green_line(matrix: &mut Vec<Vec<Tile>>, p_a: &Position, p_b: &Position) {
    run_for_line(p_a, p_b, |y, x| {
        matrix[y][x] = Tile::Green;
        false
    });

    // if p_a.x == p_b.x {
    //     let min_y = p_a.y.min(p_b.y);
    //     let max_y = p_a.y.max(p_b.y);
    //     for y in min_y + 1..max_y {
    //         matrix[y][p_a.x] = Tile::Green;
    //     }
    // } else if p_a.y == p_b.y {
    //     let min_x = p_a.x.min(p_b.x);
    //     let max_x = p_a.x.max(p_b.x);
    //     for x in min_x + 1..max_x {
    //         matrix[p_a.y][x] = Tile::Green;
    //     }
    // } else {
    //     panic!("points are not in line");
    // }
}

fn run_for_line<F: FnMut(usize, usize) -> bool>(
    p_a: &Position,
    p_b: &Position,
    mut fun: F,
) -> bool {
    if p_a.x == p_b.x {
        let min_y = p_a.y.min(p_b.y);
        let max_y = p_a.y.max(p_b.y);
        for y in min_y + 1..max_y {
            if fun(y, p_a.x) {
                return true;
            };
        }
    } else if p_a.y == p_b.y {
        let min_x = p_a.x.min(p_b.x);
        let max_x = p_a.x.max(p_b.x);
        for x in min_x + 1..max_x {
            if fun(p_a.y, x) {
                return true;
            }
        }
    } else {
        panic!("points are not in line");
    }
    false
}

fn parse_lines(data: String) -> Vec<Position> {
    data.lines()
        .map(|line| {
            let numbers: Vec<usize> = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            Position {
                x: numbers[0],
                y: numbers[1],
            }
        })
        .collect()
}

fn print_matrix(matrix: &Vec<Vec<Tile>>) {
    println!("\nmatrix:");
    return;
    for row in matrix {
        for tile in row {
            let tile = match tile {
                Tile::Empty => '.',
                Tile::Red => '#',
                Tile::Green => 'X',
            };
            print!("{}", tile);
        }
        println!();
    }
}
