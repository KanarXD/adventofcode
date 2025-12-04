use std::fmt::Debug;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Paper,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}
fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let matrix: Vec<Vec<Tile>> = parse_lines(data);
    print_matrix(&matrix, None);

    let sum = process_data(&matrix);
    println!("sum={}", sum)
}

fn process_data(matrix: &Vec<Vec<Tile>>) -> u64 {
    let mut sum = 0;

    let mut matrix_a: Vec<Vec<Tile>> = copy_matrix(matrix);
    let mut matrix_b: Vec<Vec<Tile>> = copy_matrix(matrix);

    loop {
        let (result_matrix, matrix, result) = process_matrix(matrix_a, matrix_b);
        matrix_a = result_matrix;
        matrix_b = override_matrix(matrix, &matrix_a);
        sum += result;
        if result == 0 {
            break;
        }
    }

    sum
}

fn process_matrix(
    matrix: Vec<Vec<Tile>>,
    mut result_matrix: Vec<Vec<Tile>>,
) -> (Vec<Vec<Tile>>, Vec<Vec<Tile>>, u64) {
    let mut sum = 0;

    let mut positions = vec![];

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if check_tile(&matrix, x as isize, y as isize) {
                sum += 1;
                result_matrix[y][x] = Tile::Empty;
                positions.push(Position { x, y });
            }
        }
    }

    print_matrix(&matrix, Some(&positions));

    (result_matrix, matrix, sum)
}

fn override_matrix(mut target_matrix: Vec<Vec<Tile>>, matrix: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            target_matrix[y][x] = matrix[y][x];
        }
    }
    target_matrix
}

fn copy_matrix(matrix: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut result_matrix = vec![];
    for row in matrix {
        let mut result_row = vec![];
        for tile in row {
            result_row.push(tile.clone());
        }
        result_matrix.push(result_row);
    }
    result_matrix
}

fn check_tile(matrix: &Vec<Vec<Tile>>, x: isize, y: isize) -> bool {
    if is_tile_equal_to(matrix, x, y, &Tile::Empty) {
        return false;
    }

    let to_check = vec![
        (x - 1, y),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x + 1, y + 1),
        (x, y - 1),
        (x, y + 1),
    ];

    let paper_neighbours: u32 = to_check
        .iter()
        .map(|(tx, ty)| {
            if is_tile_equal_to(matrix, *tx, *ty, &Tile::Paper) {
                return 1;
            }
            return 0;
        })
        .sum();

    // println!("x={} y={} paper_neighbours={}", x, y, paper_neighbours);

    paper_neighbours < 4
}

fn is_tile_equal_to(matrix: &Vec<Vec<Tile>>, x: isize, y: isize, tile: &Tile) -> bool {
    if y < 0 || y >= matrix.len() as isize || x < 0 || x >= matrix[0].len() as isize {
        return false;
    }
    matrix[y as usize][x as usize] == *tile
}

fn parse_lines(data: String) -> Vec<Vec<Tile>> {
    data.split("\n")
        .map(|line| {
            line.chars()
                .map(|c| {
                    return match c {
                        '.' => Tile::Empty,
                        '@' => Tile::Paper,
                        _ => panic!("bad input"),
                    };
                })
                .collect()
        })
        .collect()
}

fn print_matrix(matrix: &Vec<Vec<Tile>>, positions: Option<&Vec<Position>>) {
    println!("\nmatrix:");
    let mut result = vec![];
    for row in matrix {
        let mut row_result = vec![];
        for tile in row {
            let tile = match tile {
                Tile::Empty => '.',
                Tile::Paper => '@',
            };
            row_result.push(tile);
        }
        result.push(row_result);
    }

    if let Some(positions) = positions {
        for &Position { x, y } in positions.iter() {
            result[y][x] = 'x';
        }
    }

    for row in result {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
}
