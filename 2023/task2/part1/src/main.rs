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

fn process_data(p0: String) -> Vec<u32> {
    todo!()
}


