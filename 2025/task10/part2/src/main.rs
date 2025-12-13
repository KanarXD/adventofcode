use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::rc::Rc;
use z3::ast::Int;
use z3::{Optimize, SatResult};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Machine {
    lights: Lights,
    wirings: Vec<Wiring>,
    voltages: Voltages,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Lights {
    desired_powers: Rc<Vec<bool>>,
    powers: Vec<bool>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Wiring {
    states: Vec<usize>,
}
#[derive(Debug, Hash, PartialOrd, PartialEq, Eq, Clone)]
struct Voltages {
    desired_values: Rc<Vec<u64>>,
    values: Vec<u64>,
    score: u64,
}

// to big 23824
fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    // println!("{}", data);

    let machines = parse_lines(data);
    println!("machines = {:?}", machines);

    let sum = process_data(&machines);
    println!("sum={}", sum)
}

fn process_data(mut machines: &Vec<Machine>) -> u64 {
    println!("processing data");
    machines.iter().map(|machine| check_machine(machine)).sum()
}

fn check_machine(machine: &Machine) -> u64 {
    let solver = &Optimize::new();

    let variables: HashMap<usize, Int> = machine
        .wirings
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let variable = Int::fresh_const(format!("x_{}", i).as_str());
            solver.assert(&variable.ge(&Int::from_u64(0)));
            (i, variable)
        })
        .collect();

    machine
        .voltages
        .desired_values
        .iter()
        .enumerate()
        .for_each(|(d_index, desired_value)| {
            let sum = machine
                .wirings
                .iter()
                .enumerate()
                .filter(|(_, wiring)| wiring.states.contains(&d_index))
                .map(|(w_index, _)| variables.get(&w_index).unwrap())
                .fold(Int::from_u64(0), |acc, v| acc + v);

            let desired_value = Int::from_u64(*desired_value);
            solver.assert(&sum.eq(desired_value));
        });

    let variables_sum = variables.values().fold(Int::from_u64(0), |acc, v| acc + v);
    solver.minimize(&variables_sum);

    println!("\nSolver: {:?}", solver);
    match solver.check(&[]) {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();

            let results: HashMap<usize, u64> = variables
                .iter()
                .map(|(index, variable)| {
                    let result = model.eval(variable, true).unwrap();
                    let result = result.as_u64().unwrap();
                    (*index, result)
                })
                .collect();
            let sum = results.values().sum::<u64>();
            println!("Sum: {sum}, Results: {:?}", results);

            return sum;
        }
        _ => {
            panic!("No solution for equation");
        }
    }

    0
}

fn parse_lines(data: String) -> Vec<Machine> {
    data.lines()
        .map(|line| {
            let entities = line.split_whitespace().collect::<Vec<&str>>();
            let lights = entities[0];
            let wirings = entities[1..entities.len() - 1].to_vec();
            let voltages = entities[entities.len() - 1];

            let lights: Vec<bool> = lights
                .chars()
                .into_iter()
                .filter(|c| !vec!['[', ']'].contains(c))
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Unknown char: {}", c),
                })
                .collect();
            let powers = lights.iter().map(|_| false).collect::<Vec<bool>>();
            let lights = Lights {
                desired_powers: Rc::new(lights),
                powers,
            };

            let wirings: Vec<Wiring> = wirings
                .into_iter()
                .map(|wiring| {
                    let wiring = &wiring[1..wiring.len() - 1];
                    let wiring: Vec<usize> = wiring
                        .split(",")
                        .map(|number| number.parse().unwrap())
                        .collect();
                    Wiring { states: wiring }
                })
                .collect();

            let voltages: Vec<u64> = voltages[1..voltages.len() - 1]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            let actual_voltages = voltages.iter().map(|_| 0).collect::<Vec<u64>>();

            let voltages = Voltages {
                desired_values: Rc::new(voltages),
                values: actual_voltages,
                score: 0,
            };

            Machine {
                lights,
                wirings,
                voltages,
            }
        })
        .collect()
}
