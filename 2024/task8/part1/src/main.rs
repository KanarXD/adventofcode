use regex::Regex;
use std::fmt::Debug;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Location {
    Empty,
    Antenna(char),
}

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let locations: Vec<Vec<Location>> = parse_lines(data);
    print_matrix(&locations);
    let width = locations[0].len();
    let height = locations.len();
    println!("width: {}, height: {}", width, height);
    // println!("{:?}", locations);

    let sum = process_data(&locations);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<Vec<Location>> {
    let regex = Regex::new(r"[a-zA-Z0-9]").unwrap();
    data.split("\n")
        .map(|line| {
            line.chars()
                .map(|location_char| {
                    let matches = regex.is_match(location_char.to_string().as_str());
                    return if matches {
                        Location::Antenna(location_char)
                    } else {
                        Location::Empty
                    };
                })
                .collect()
        })
        .collect()
}

fn process_data(locations: &Vec<Vec<Location>>) -> u32 {
    let mut antinodes: Vec<Vec<bool>> = locations
        .iter()
        .map(|line| line.iter().map(|x| false).collect())
        .collect();

    let width = locations[0].len();
    let height = locations.len();

    for y in 0..height {
        for x in 0..width {
            if let Location::Antenna(character) = locations[y][x] {
                find_antinodes(locations, &mut antinodes, character, x as isize, y as isize);
            }
        }
    }

    // println!("{:?}", antinodes);

    print_matrix(&antinodes);

    antinodes
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count() as u32)
        .sum()
}

fn find_antinodes(
    locations: &Vec<Vec<Location>>,
    mut antinodes: &mut Vec<Vec<bool>>,
    antenna_character: char,
    antenna_x: isize,
    antenna_y: isize,
) {
    let width = locations[0].len() as isize;
    let height = locations.len() as isize;

    for y in 0..height {
        for x in 0..width {
            if let Location::Antenna(character) = locations[y as usize][x as usize] {
                if character != antenna_character || (antenna_x == x && antenna_y == y) {
                    continue;
                }

                let delta_x = x - antenna_x;
                let delta_y = y - antenna_y;

                let first_antinode_x = antenna_x - delta_x;
                let second_antinode_x = x + delta_x;
                let first_antinode_y = antenna_y - delta_y;
                let second_antinode_y = y + delta_y;

                mark_location_antinode(&mut antinodes, first_antinode_x, first_antinode_y);
                mark_location_antinode(&mut antinodes, second_antinode_x, second_antinode_y);
            }
        }
    }
}

fn mark_location_antinode(antinodes: &mut Vec<Vec<bool>>, x: isize, y: isize) {
    let width = antinodes[0].len() as isize;
    let height = antinodes.len() as isize;
    if x < 0 || y < 0 || x >= width || y >= height {
        return;
    }
    antinodes[y as usize][x as usize] = true;
}

fn print_matrix<T: Debug>(matrix: &Vec<Vec<T>>) {
    matrix.iter().for_each(|row| {
        println!("{:?}", row);
    })
}
