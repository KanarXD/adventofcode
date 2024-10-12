use std::char::from_digit;
use std::cmp::Ordering;
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
    let digit_chars = "0,1,2,3,4,5,6,7,8,9"
        .split(",")
        .collect::<Vec<&str>>();

    let digit_words = "zero,one,two,three,four,five,six,seven,eight,nine"
        .split(",")
        .collect::<Vec<&str>>();

    data.split('\n')
        .map(|line| {
            let (first_word, last_word) = find_first_and_last(&digit_words, line);
            let (first_char, last_char) = find_first_and_last(&digit_chars, line);

            let first_digit = find_first_digit(first_word, first_char);
            let last_digit = find_last_digit(last_word, last_char);

            let slice = [first_digit, last_digit];
            slice.iter().collect::<String>()
        })
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

fn find_first_and_last<>(digits: &Vec<&str>, line: &str) -> (Option<PositionDigit>, Option<PositionDigit>) {
    let mut line_char_digits: Vec<PositionDigit> = search_digits_in_line(line, digits);
    let min_char: Option<PositionDigit> = line_char_digits.iter().min_by(position_digit_comparator).copied();
    let max_char: Option<PositionDigit> = line_char_digits.iter().max_by(position_digit_comparator).copied();
    (min_char, max_char)
}

fn find_last_digit(max_word: Option<PositionDigit>, max_char: Option<PositionDigit>) -> char {
    let last_digit = match (max_word, max_char) {
        (None, Some((_, digit))) => digit,
        (Some((_, digit)), None) => digit,
        (Some((word_position, word_digit)), Some((char_position, char_digit))) if char_position > word_position => char_digit,
        (Some((word_position, word_digit)), Some((char_position, char_digit))) if char_position < word_position => word_digit,
        _ => panic!("no digits in line")
    };
    from_digit(last_digit as u32, 10)
        .expect("last_digit has to convert to char")
}

fn find_first_digit(min_word: Option<PositionDigit>, min_char: Option<PositionDigit>) -> char {
    let first_digit = match (min_word, min_char) {
        (None, Some((_, digit))) => digit,
        (Some((_, digit)), None) => digit,
        (Some((word_position, word_digit)), Some((char_position, char_digit))) if char_position < word_position => char_digit,
        (Some((word_position, word_digit)), Some((char_position, char_digit))) if char_position > word_position => word_digit,
        _ => panic!("no digits in line"),
    };
    from_digit(first_digit as u32, 10).expect("first_digit has to convert to char")
}

fn search_digits_in_line(line: &str, digits: &Vec<&str>) -> Vec<PositionDigit> {
    digits.iter()
        .enumerate()
        .flat_map(|(digit, number_string)| {
            line.match_indices(*number_string)
                .map(|(x, _)| (x, digit))
                .collect::<Vec<PositionDigit>>()
        })
        .collect::<Vec<PositionDigit>>()
}

fn position_digit_comparator((position1, _): &&PositionDigit, (position2, _): &&PositionDigit) -> Ordering {
    position1.cmp(position2)
}
