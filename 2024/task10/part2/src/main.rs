use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let locations: Vec<Vec<u32>> = parse_lines(data);
    println!("result: {:?}", locations);

    let sum = process_data(&locations);
    println!("sum={}", sum)
}

fn parse_lines(data: String) -> Vec<Vec<u32>> {
    data.split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn process_data(locations: &Vec<Vec<u32>>) -> u32 {
    let width = locations[0].len();
    let height = locations.len();
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if locations[y][x] != 0 {
                continue;
            }
            let mut visited = HashSet::new();
            sum += find_nine(locations, &mut visited, x as isize, y as isize, 0);
        }
    }
    sum
}

fn find_nine(
    locations: &Vec<Vec<u32>>,
    mut visited: &mut HashSet<(isize, isize)>,
    x: isize,
    y: isize,
    desired_height: u32,
) -> u32 {
    let width = locations[0].len() as isize;
    let height = locations.len() as isize;

    let mut sum = 0;
    let current_location = locations[y as usize][x as usize];

    if current_location != desired_height {
        return 0;
    }

    if current_location == 9 {
        let position = (x, y);
        return 1;
        // return if visited.contains(&position) {
        //     0
        // } else {
        //     visited.insert(position);
        //     1
        // };
    }

    let next_desired_height = desired_height + 1;
    if x - 1 >= 0 {
        sum += find_nine(locations, &mut visited, x - 1, y, next_desired_height);
    }
    if y - 1 >= 0 {
        sum += find_nine(locations, &mut visited, x, y - 1, next_desired_height);
    }
    if x + 1 < width {
        sum += find_nine(locations, &mut visited, x + 1, y, next_desired_height);
    }
    if y + 1 < height {
        sum += find_nine(locations, &mut visited, x, y + 1, next_desired_height);
    }

    sum
}
