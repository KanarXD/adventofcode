use std::fmt::Debug;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Range {
    left: u64,
    right: u64,
}

impl Range {
    fn contains(&self, x: u64) -> bool {
        x >= self.left && x <= self.right
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let (ranges, ids) = parse_lines(data);
    println!("ranges = {:?}", ranges);
    println!("ids = {:?}", ids);

    let sum = process_data(&ranges, &ids);
    println!("sum={}", sum)
}

fn process_data(ranges: &Vec<Range>, ids: &Vec<u64>) -> u64 {
    let mut sum = 0;

    for &id in ids {
        'ranges: for range in ranges {
            if range.contains(id) {
                sum += 1;
                println!("range contains {}", id);
                break 'ranges;
            }
        }
    }

    sum
}

fn parse_lines(data: String) -> (Vec<Range>, Vec<u64>) {
    let parts: Vec<&str> = data.split("\n\n").collect();
    let ranges = *parts.get(0).unwrap();
    let ids = *parts.get(1).unwrap();
    let ranges = ranges
        .split("\n")
        .map(|range| {
            let parts: Vec<&str> = range.split("-").collect();
            let left: u64 = parts[0].parse().unwrap();
            let right: u64 = parts[1].parse().unwrap();
            Range { left, right }
        })
        .collect();

    let ids = ids.split("\n").map(|id| id.parse().unwrap()).collect();

    (ranges, ids)
}
