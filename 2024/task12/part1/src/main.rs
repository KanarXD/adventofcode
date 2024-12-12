use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;

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

                let borders =
                    check_region(locations, letter, &mut checked_regions_single, position);

                let area = checked_regions_single.len() as u64;
                let price = borders * area;
                println!("letter={letter}, borders={borders}, area={area}, price={price}");

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
    position: Position,
) -> u64 {
    if let Some(region_letter) = check_location(locations, &position) {
        if checked_regions.contains(&position) {
            return 0;
        }
        if region_letter != letter {
            return 1;
        }

        checked_regions.insert(position);

        let mut sum = 0;
        sum += check_region(
            locations,
            letter,
            &mut checked_regions,
            Position {
                x: position.x - 1,
                y: position.y,
            },
        );
        sum += check_region(
            locations,
            letter,
            &mut checked_regions,
            Position {
                x: position.x,
                y: position.y - 1,
            },
        );
        sum += check_region(
            locations,
            letter,
            &mut checked_regions,
            Position {
                x: position.x + 1,
                y: position.y,
            },
        );
        sum += check_region(
            locations,
            letter,
            &mut checked_regions,
            Position {
                x: position.x,
                y: position.y + 1,
            },
        );
        return sum;
    }
    return 1;
}
