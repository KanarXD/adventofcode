use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Splitter,
    ShotSplitter,
    Start,
    Beam,
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let tiles = parse_lines(data);
    println!("tiles = {:?}", tiles);
    print_matrix(&tiles);

    let sum = process_data(tiles);
    println!("sum={}", sum)
}

fn process_data(mut tiles: Vec<Vec<Tile>>) -> u64 {
    let mut sum = 0;

    let height = tiles.len();
    let width = tiles[0].len();
    for y in 0..height - 1 {
        for x in 0..width {
            let tile = &tiles[y][x];
            match tile {
                Tile::Empty => {}
                Tile::Splitter => {}
                Tile::Start => {
                    tiles[y + 1][x] = Tile::Beam;
                }
                Tile::Beam => {
                    let bottom_tile = &mut tiles[y + 1][x];
                    match bottom_tile {
                        Tile::Empty => {
                            *bottom_tile = Tile::Beam;
                        }
                        Tile::Splitter => {
                            *bottom_tile = Tile::ShotSplitter;
                            tiles[y + 1][x - 1] = Tile::Beam;
                            tiles[y + 1][x + 1] = Tile::Beam;
                        }
                        Tile::Start => panic!("Start was shot"),
                        Tile::ShotSplitter => {}
                        Tile::Beam => {}
                    }
                }
                Tile::ShotSplitter => {
                    sum += 1;
                    tiles[y][x - 1] = Tile::Beam;
                    tiles[y][x + 1] = Tile::Beam;
                }
            }
        }
    }

    print_matrix(&tiles);
    sum
}

fn parse_lines(data: String) -> Vec<Vec<Tile>> {
    let mut tiles = Vec::new();
    data.lines().for_each(|line| {
        let mut row = Vec::new();
        for tile in line.chars() {
            match tile {
                '.' => row.push(Tile::Empty),
                '^' => row.push(Tile::Splitter),
                'S' => row.push(Tile::Start),
                _ => panic!("Invalid tile: {tile:?}"),
            }
        }
        tiles.push(row);
    });

    tiles
}

fn print_matrix(matrix: &Vec<Vec<Tile>>) {
    println!("\nmatrix:");
    let mut result = vec![];
    for row in matrix {
        let mut row_result = vec![];
        for tile in row {
            let tile = match tile {
                Tile::Empty => '.',
                Tile::Splitter => '^',
                Tile::Start => 'S',
                Tile::Beam => '|',
                Tile::ShotSplitter => '@',
            };
            row_result.push(tile);
        }
        result.push(row_result);
    }

    for row in result {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
}
