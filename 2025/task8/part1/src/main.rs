use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn distance(&self, position: &Position) -> u64 {
        ((self.x - position.x).pow(2) + (self.y - position.y).pow(2) + (self.z - position.z).pow(2))
            as u64
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let positions = parse_lines(data);
    // println!("positions = {:?}", positions);

    let sum = process_data(positions);
    println!("sum={}", sum)
}

fn process_data(mut positions: Vec<Position>) -> u64 {
    let distances = get_distances(&positions);
    // println!("Distances = {:?}", distances);

    let iterations = 1000;
    let mut used_points: HashMap<Position, usize> = HashMap::new();
    let mut circuits: HashMap<usize, Vec<Position>> = HashMap::new();

    for (key, &(p1, p2)) in distances.values().flat_map(|edge| edge).enumerate() {
        if key >= iterations {
            break;
        }
        match (used_points.get(&p1), used_points.get(&p2)) {
            (Some(&p1_value), Some(&p2_value)) => {
                if p1_value == p2_value {
                    continue;
                }
                let mut p2_positions = circuits.remove(&p2_value).unwrap();
                let p1_positions = circuits.get_mut(&p1_value).unwrap();
                p1_positions.append(&mut p2_positions);
                p1_positions.iter().for_each(|&p1_pos| {
                    used_points.insert(p1_pos, p1_value);
                });
            }
            (None, None) => {
                circuits.insert(key, vec![p1, p2]);
                used_points.insert(p1, key);
                used_points.insert(p2, key);
            }
            (None, Some(&p2_value)) => {
                let positions = circuits.get_mut(&p2_value).unwrap();
                used_points.insert(p1, p2_value);
                positions.push(p1);
            }
            (Some(&p1_value), None) => {
                let positions = circuits.get_mut(&p1_value).unwrap();
                used_points.insert(p2, p1_value);
                positions.push(p2);
            }
        }
    }

    let circuits_sizes: BTreeMap<usize, Vec<Position>> = circuits
        .into_iter()
        .map(|(_, positions)| (positions.len(), positions))
        .collect();

    println!("circuits_sizes = {:?}", circuits_sizes);

    circuits_sizes
        .keys()
        .rev()
        .take(3)
        .fold(1, |acc, &pos| acc * (pos as u64))
}

fn get_distances(positions: &Vec<Position>) -> BTreeMap<u64, Vec<(Position, Position)>> {
    let mut distances: BTreeMap<u64, Vec<(Position, Position)>> = BTreeMap::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let position_a = positions[i];
            let position_b = positions[j];
            let distance = position_a.distance(&position_b);
            // distances.insert((position_a, position_b), distance);

            distances
                .entry(distance)
                .or_insert_with(Vec::new)
                .push((position_a, position_b));
        }
    }
    distances
}

fn parse_lines(data: String) -> Vec<Position> {
    data.lines()
        .map(|line| {
            let numbers: Vec<i64> = line.split(',').map(|n| n.parse::<i64>().unwrap()).collect();
            Position {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect()
}
