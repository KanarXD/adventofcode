use std::fmt::Debug;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Paper,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}
fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let matrix: Vec<Vec<Tile>> = parse_lines(data);
    // println!("matrix: {:?}", matrix);
    // print_matrix(&matrix);

    let sum = process_data(&matrix);
    println!("sum={}", sum)
}

fn process_data(matrix: &Vec<Vec<Tile>>) -> u64 {
    let mut sum = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if check_tile(matrix, x as isize, y as isize) {
                sum += 1
            }
        }
    }

    sum
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

fn print_matrix(matrix: &Vec<Vec<Tile>>) {
    for row in matrix {
        for tile in row {
            match tile {
                Tile::Empty => print!("."),
                Tile::Paper => print!("@"),
            }
        }
        println!();
    }
}
