use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Occupied,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Matrix {
    tiles: Vec<Vec<Tile>>,
}
impl Matrix {
    fn new(height: usize, width: usize) -> Matrix {
        let tiles = vec![vec![Tile::Empty; width]; height];
        Matrix { tiles }
    }
    fn rotate_anti_clockwise(&self) -> Matrix {
        self.mirror_horizontal().rotate_clockwise()
    }
    fn rotate_clockwise(&self) -> Matrix {
        let mut tiles = vec![];
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        for x in 0..width {
            let mut row = vec![];
            for y in 0..height {
                row.push(self.tiles[y][x]);
            }
            tiles.push(row);
        }
        Matrix { tiles }
    }
    fn mirror_horizontal(&self) -> Matrix {
        let mut tiles = vec![];
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        for y in (0..height).rev() {
            let mut row = vec![];
            for x in (0..width).rev() {
                row.push(self.tiles[y][x]);
            }
            tiles.push(row);
        }
        Matrix { tiles }
    }
    fn add_matrix(&mut self, matrix: &Matrix) -> Option<Matrix> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Present {
    index: usize,
    matrices: Vec<Matrix>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

impl Region {
    fn fits_presents(&self, presents: &HashMap<usize, Present>) -> bool {
        let matrix = Matrix::new(self.height, self.width);
        self.dfs(&matrix, presents, self.presents.clone())
    }

    fn dfs(
        &self,
        matrix: &Matrix,
        presents: &HashMap<usize, Present>,
        left_presents: Vec<usize>,
    ) -> bool {
        let Some((index, count)) = left_presents.iter().enumerate().find(|&(_, v)| *v > 0) else {
            return true;
        };

        todo!()
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    // let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let (presents, regions) = parse_lines(&data);
    println!("presents = {:#?}", presents);
    println!("regions = {:?}", regions);

    let sum = process_data(&presents, &regions);
    println!("sum={}", sum)
}

fn process_data(presents: &HashMap<usize, Present>, regions: &Vec<Region>) -> u64 {
    println!("processing data");

    regions
        .iter()
        .filter(|region| region.fits_presents(presents))
        .count() as u64
}

fn parse_lines(data: &String) -> (HashMap<usize, Present>, Vec<Region>) {
    let mut lines = data.split("\n\n").collect::<Vec<&str>>();

    let regions = lines.pop().unwrap();
    let presents = lines;

    let presents = presents
        .iter()
        .map(|section| {
            let s_lines = section.lines().collect::<Vec<&str>>();
            let index_line = s_lines[0];
            let index = index_line[..index_line.len() - 1].parse::<usize>().unwrap();
            let tiles = s_lines[1..s_lines.len()]
                .into_iter()
                .map(|tile_line| {
                    tile_line
                        .chars()
                        .map(|c| match c {
                            '.' => Tile::Empty,
                            '#' => Tile::Occupied,
                            _ => panic!("unknown tile: {}", c),
                        })
                        .collect::<Vec<Tile>>()
                })
                .collect::<Vec<Vec<Tile>>>();

            let matrix = Matrix { tiles };

            let matrices = vec![
                matrix.mirror_horizontal(),
                matrix.rotate_clockwise(),
                matrix.rotate_anti_clockwise(),
                matrix,
            ];

            let present = Present { index, matrices };
            (index, present)
        })
        .collect::<HashMap<usize, Present>>();

    let regions = regions
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let dimensions = parts[0].split("x").collect::<Vec<&str>>();
            let presents_requirements = parts[1]
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect();

            let width = dimensions[0].parse::<usize>().unwrap();
            let height = dimensions[1].parse::<usize>().unwrap();

            Region {
                width,
                height,
                presents: presents_requirements,
            }
        })
        .collect::<Vec<Region>>();

    (presents, regions)
}
