use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

type Digit = usize;
type Position = usize;
type PositionDigit = (Position, Digit);

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String = fs::read_to_string(file_path)
        .expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);


    let numbers = process_data(data);
    let sum = numbers.iter().sum::<u32>();

    println!("{:?}", numbers);
    println!("sum={}", sum);
}

fn process_data(data: String) -> Vec<u32> {
    let digit_words_map: HashMap<&str, &str> = HashMap::from([
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let digit_words: HashSet<&str> = digit_words_map.keys().copied().collect();
    data.split('\n')
        .map(|line| {
            let tokens = parse_tokens(line, &digit_words);
            // println!("line= {}, tokens={:?}", line, tokens);
            let first = tokens.first().expect("line has to have first digit").as_str();
            let last = tokens.last().expect("line has to have last digit").as_str();

            let first_digit = *digit_words_map.get(first).expect("first digit has to convert");
            let last_digit = *digit_words_map.get(last).expect("last digit has to convert");

            format!("{first_digit}{last_digit}")
        })
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}
fn parse_tokens(string: &str, tokens: &HashSet<&str>) -> Vec<String> {
    let mut length_tokens: BTreeMap<usize, HashSet<&str>> = BTreeMap::new();
    tokens.iter()
        .for_each(|&token| {
            length_tokens.entry(token.len())
                .or_insert_with(HashSet::new)
                .insert(token);
        });
    let mut found_tokens = vec![];
    let mut i = 0;
    while i < string.len() {
        for (length, tokens) in &length_tokens {
            if i + length > string.len() {
                break;
            }
            let slice = &string[i..i + length];
            if tokens.contains(slice) {
                i += slice.len() - 1;
                found_tokens.push(slice.to_string());
                break;
            }
        }
        i += 1;
    }
    found_tokens
}
