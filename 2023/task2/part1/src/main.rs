use std::fs;

fn main() {
    let file_path = "res/demo_input.txt";
    // let file_path = "res/input.txt";

    let data: String = fs::read_to_string(file_path)
        .expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let numbers: Vec<u32> = process_data(data);
    let sum: u32 = numbers.iter().sum::<u32>();

    println!("{:?}", numbers);
    println!("sum={}", sum)
}

fn process_data(data: String) -> Vec<u32> {
    data.split("\n").map(|line| {
        let tokens: Vec<&str> = line.split(" ").collect();

        let id_token = tokens[1];
        let id = &id_token[0..id_token.len() - 1];


        return id;
    })
        .map(|number| number.parse::<u32>().expect(format!("{number} is not a number").as_str()))
        .collect()
}


