use std::collections::VecDeque;
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
    fn overlaps(&self, range: &Range) -> bool {
        self.contains(range.left)
            || self.contains(range.right)
            || range.contains(self.left)
            || range.contains(self.right)
    }
    fn add_range(&self, range: &Range) -> Self {
        let left = self.left.min(range.left);
        let right = self.right.max(range.right);
        Range { left, right }
    }
    fn points(&self) -> u64 {
        self.right - self.left + 1
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let (ranges, ids) = parse_lines(data);
    println!("ranges={:?}", ranges);
    println!("ids = {:?}", ids);

    let sum = process_data(ranges);
    println!("sum={}", sum)
}

fn process_data(mut ranges: VecDeque<Range>) -> u64 {
    loop {
        let last_size = ranges.len();
        for i in 0..last_size {
            let base_range = ranges.pop_front().unwrap();
            ranges = check_range(base_range, ranges);
            println!("i={i} ranges={ranges:?}");
        }

        if ranges.len() >= last_size {
            break;
        }
    }

    ranges.iter().map(|r| r.points()).sum::<u64>()
}

fn check_range(mut base_range: Range, ranges: VecDeque<Range>) -> VecDeque<Range> {
    let mut result: VecDeque<Range> = ranges
        .into_iter()
        .map(|range| {
            if base_range.overlaps(&range) {
                base_range = base_range.add_range(&range);
                None
            } else {
                Some(range)
            }
        })
        .flatten()
        .collect();

    result.push_back(base_range);

    result
}

fn parse_lines(data: String) -> (VecDeque<Range>, Vec<u64>) {
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
