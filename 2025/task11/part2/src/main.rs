use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Device<'t> {
    name: Name<'t>,
    outputs: Vec<Name<'t>>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

    let paths = [
        [["svr", "fft"], ["fft", "dac"], ["dac", "out"]],
        [["svr", "dac"], ["dac", "fft"], ["fft", "out"]],
    ];

    paths
        .map(|path| {
            path.map(|[start, end]| {
                run_dfs(devices, devices.get(&Name(start)).unwrap(), &Name(end))
            })
            .iter()
            .fold(1, |acc, dir| acc * dir)
        })
        .iter()
        .sum()
}

fn run_dfs(devices: &HashMap<Name, Device>, device: &Device, target: &Name) -> u64 {
    let mut visited = HashMap::new();
    dfs(devices, device, target, &mut visited)
}

fn dfs<'t>(
    devices: &HashMap<Name<'t>, Device<'t>>,
    device: &Device<'t>,
    target: &Name<'t>,
    visited: &mut HashMap<Name<'t>, u64>,
) -> u64 {
    if visited.contains_key(&device.name) {
        return visited[&device.name];
    }
    if device.name == *target {
        println!("found target: {}", target.0);
        visited.insert(device.name, 1);
        return 1;
    }
    let count = device
        .outputs
        .iter()
        .map(|name| {
            let output_device = devices.get(name).unwrap();
            dfs(devices, output_device, target, visited)
        })
        .sum();

    visited.insert(device.name, count);

    count
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
