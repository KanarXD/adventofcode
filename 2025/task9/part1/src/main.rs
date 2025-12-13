use std::collections::BTreeMap;
use std::fs;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn rect_size(&self, position: &Position) -> u64 {
        (self.y.abs_diff(position.y) + 1) * (self.x.abs_diff(position.x) + 1)
    }
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

fn process_data(mut positions: Vec<Position>) -> u64 {
    let mut rects: BTreeMap<u64, Vec<(Position, Position)>> = BTreeMap::new();

    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let position_a = positions[i];
            let position_b = positions[j];
            let rect_size = position_a.rect_size(&position_b);
            rects
                .entry(rect_size)
                .or_insert(Vec::new())
                .push((position_a, position_b));
        }
    }

    // println!("rects={:?}", rects);
    *rects.keys().last().unwrap()
}

fn parse_lines(data: String) -> Vec<Position> {
    data.lines()
        .map(|line| {
            let numbers: Vec<u64> = line.split(',').map(|n| n.parse::<u64>().unwrap()).collect();
            Position {
                x: numbers[0],
                y: numbers[1],
            }
        })
        .collect()
}
