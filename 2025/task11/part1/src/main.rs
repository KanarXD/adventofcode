use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Device<'t> {
    name: Name<'t>,
    outputs: Vec<Name<'t>>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Name<'t>(&'t str);

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let devices = parse_lines(&data);
    println!("devices = {:?}", devices);

    let sum = process_data(&devices);
    println!("sum={}", sum)
}

fn process_data(devices: &HashMap<Name, Device>) -> u64 {
    println!("processing data");
    let device = devices.get(&Name("you")).unwrap();

    dfs(devices, device)
}

fn dfs(devices: &HashMap<Name, Device>, device: &Device) -> u64 {
    if device.name == Name("out") {
        return 1;
    }
    device
        .outputs
        .iter()
        .map(|name| {
            let output_device = devices.get(name).unwrap();
            dfs(devices, output_device)
        })
        .sum()
}

fn parse_lines(data: &String) -> HashMap<Name, Device> {
    let mut devices: HashMap<Name, Device> = data
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let device_name = parts[0];
            let outputs = parts[1]
                .split_whitespace()
                .map(|s| Name(s))
                .collect::<Vec<Name>>();

            let name = Name(device_name);
            let device = Device {
                name: name.clone(),
                outputs,
            };
            (name, device)
        })
        .collect();

    devices.insert(
        Name("out"),
        Device {
            name: Name("out"),
            outputs: vec![],
        },
    );

    devices
}
