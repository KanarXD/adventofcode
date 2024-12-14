use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Edge {
    vertical: bool,
    start: Position,
    end: Position,
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

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
        // if letter != 'E' {
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
                // let mut borders: u64 = checked_edges
                //     .iter()
                //     .map(|edge| bonus_borders(locations, letter, edge))
                //     .sum::<u64>();
                let mut borders: u64 = checked_edges.len() as u64;

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

fn insert_edge(
    locations: &Vec<Vec<char>>,
    letter: char,
    mut checked_edges: &mut HashSet<Edge>,
    position: &Position,
    mut edge: Edge,
) {
    let width = locations[0].len() as isize;
    let height = locations.len() as isize;
    if edge.vertical {
        for x in position.x + 1..=width {
            edge.start.x = x - 1;
            edge.end.x = x - 1;
            // let checked_letter = locations[edge.start.y as usize][x as usize];
            if let Some(checked_letter) =
                check_location(locations, &Position { y: edge.start.y, x })
            {
                if checked_letter == letter {
                    if let Some(end_letter) =
                        check_location(locations, &Position { y: edge.end.y, x })
                    {
                        if end_letter == letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    } else {
        for y in position.y + 1..=height {
            // let checked_letter = locations[y as usize][edge.start.x as usize];
            edge.start.y = y - 1;
            edge.end.y = y - 1;
            if let Some(checked_letter) =
                check_location(locations, &Position { y, x: edge.start.x })
            {
                if checked_letter == letter {
                    if let Some(end_letter) =
                        check_location(locations, &Position { y, x: edge.end.x })
                    {
                        if end_letter == letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }
    checked_edges.insert(edge);
    // panic!("failed to insert edge");
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
            let edge = create_edge(previous_position, &position);
            insert_edge(locations, letter, checked_edges, &position, edge);
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
    } else {
        let edge = create_edge(previous_position, &position);
        insert_edge(locations, letter, checked_edges, &position, edge);
    }
}

fn create_edge(previous_position: &Position, position: &Position) -> Edge {
    if previous_position.y_coordinate_equals(&position) {
        let edge = Edge {
            vertical: false,
            start: *previous_position,
            end: *position,
        };
        return edge;
    } else if previous_position.x_coordinate_equals(&position) {
        let edge = Edge {
            vertical: true,
            start: *previous_position,
            end: *position,
        };
        return edge;
    }
    panic!("Something went wrong");
}
