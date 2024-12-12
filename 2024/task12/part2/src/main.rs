use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    y: isize,
    x: isize,
}

impl Position {
    fn x_coordinate_equals(&self, position: &Position) -> bool {
        self.x == position.x
    }

    fn y_coordinate_equals(&self, position: &Position) -> bool {
        self.y == position.y
    }

    fn x_difference(&self, position: &Position) -> isize {
        self.x - position.x
    }

    fn y_difference(&self, position: &Position) -> isize {
        self.y - position.y
    }
}

// #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[derive(Debug, Clone, Copy)]
struct Edge {
    start: Position,
    end: Position,
}

// impl Edge {
//     fn on_the_same_edge(&self, edge: Edge) -> bool {
//         (self.start.x_coordinate_equals(edge.start) && self.end.x_coordinate_equals(edge.end)) ||
//         (self.start.y_coordinate_equals(edge.start) && self.end.y_coordinate_equals(edge.end))
//     }
// }

impl PartialEq<Self> for Edge {
    fn eq(&self, edge: &Self) -> bool {
        (
            self.start.x_coordinate_equals(&edge.start) && self.end.x_coordinate_equals(&edge.end)
            // && self.start.y_difference(&edge.start) == self.end.y_difference(&edge.end)
        ) || (
            self.start.y_coordinate_equals(&edge.start) && self.end.y_coordinate_equals(&edge.end)
            // && self.start.x_difference(&edge.start) == self.end.x_difference(&edge.end)
        )
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.start.x_coordinate_equals(&self.end) {
            self.start.y.hash(state);
        } else if self.start.y_coordinate_equals(&self.end) {
            self.start.x.hash(state);
        } else {
            panic!("bad edge");
        }
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    // let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let (locations, letters): (Vec<Vec<char>>, HashSet<char>) = parse_lines(data);
    println!("locations: {:?}", locations);
    println!("letters: {:?}", letters);

    let sum = process_data(&locations, &letters);
    println!("sum={}", sum)
}

fn check_location(locations: &Vec<Vec<char>>, position: &Position) -> Option<char> {
    if let Some(line) = locations.get(position.y as usize) {
        if let Some(letter) = line.get(position.x as usize) {
            return Some(*letter);
        }
    }
    None
}

fn parse_lines(data: String) -> (Vec<Vec<char>>, HashSet<char>) {
    let letters = data.chars().filter(|&l| l != '\n').collect();
    let locations = data
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    (locations, letters)
}

fn process_data(locations: &Vec<Vec<char>>, letters: &HashSet<char>) -> u64 {
    let mut sum = 0;
    let width = locations[0].len();
    let height = locations.len();
    for &letter in letters {
        // if letter != 'C' {
        //     continue;
        // }
        let mut checked_regions: HashSet<Position> = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                let checked_letter = locations[y][x];
                let position = Position {
                    x: x as isize,
                    y: y as isize,
                };
                if checked_letter != letter || checked_regions.contains(&position) {
                    continue;
                }
                let mut checked_regions_single: HashSet<Position> = HashSet::new();
                let mut checked_edges: HashSet<Edge> = HashSet::new();

                check_region(
                    locations,
                    letter,
                    &mut checked_regions_single,
                    &mut checked_edges,
                    &position,
                    position.clone(),
                );
                let borders: u64 = checked_edges.len() as u64;

                let area = checked_regions_single.len() as u64;
                let price = borders * area;
                println!("letter={letter}, borders={borders}, area={area}, price={price}");
                println!("edges={:?}", checked_edges);

                sum += price;
                checked_regions.extend(checked_regions_single);
            }
        }
    }
    sum
}

fn check_region(
    locations: &Vec<Vec<char>>,
    letter: char,
    mut checked_regions: &mut HashSet<Position>,
    mut checked_edges: &mut HashSet<Edge>,
    previous_position: &Position,
    position: Position,
) {
    // println!("position={position:?}");
    if let Some(region_letter) = check_location(locations, &position) {
        if checked_regions.contains(&position) {
            return;
        }
        if region_letter != letter {
            let edge = Edge {
                start: previous_position.clone(),
                end: position,
            };
            checked_edges.insert(edge);
            return;
        }

        checked_regions.insert(position);

        check_region(
            locations,
            letter,
            &mut checked_regions,
            &mut checked_edges,
            &position,
            Position {
                x: position.x - 1,
                y: position.y,
            },
        );
        check_region(
            locations,
            letter,
            &mut checked_regions,
            &mut checked_edges,
            &position,
            Position {
                x: position.x,
                y: position.y - 1,
            },
        );
        check_region(
            locations,
            letter,
            &mut checked_regions,
            &mut checked_edges,
            &position,
            Position {
                x: position.x + 1,
                y: position.y,
            },
        );
        check_region(
            locations,
            letter,
            &mut checked_regions,
            &mut checked_edges,
            &position,
            Position {
                x: position.x,
                y: position.y + 1,
            },
        );
        return;
    }

    let edge = Edge {
        start: previous_position.clone(),
        end: position,
    };
    checked_edges.insert(edge);
}
