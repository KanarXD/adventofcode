use std::collections::VecDeque;
use std::fs;
use std::hash::Hash;
use std::rc::Rc;

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

impl Lights {
    fn is_desired(&self) -> bool {
        for (desired, power) in self.desired_powers.iter().zip(self.powers.iter()) {
            if desired != power {
                return false;
            }
        }
        true
    }
}

impl Lights {
    fn apply_wiring(&self, wiring: &Wiring) -> Lights {
        let mut new_powers = self.powers.clone();
        for &state in wiring.states.iter() {
            new_powers[state] = !new_powers[state];
        }

        Lights {
            desired_powers: self.desired_powers.clone(),
            powers: new_powers,
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum PowerState {
    On,
    Off,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Wiring {
    states: Vec<usize>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Voltages {
    values: Vec<usize>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Calculation {
    lights: Lights,
    steps: u64,
}

impl Calculation {
    fn apply_wire(&self, wire: &Wiring) -> Calculation {
        Calculation {
            lights: self.lights.apply_wiring(wire),
            steps: self.steps + 1,
        }
    }

    fn is_ready(&self) -> bool {
        self.lights.is_desired()
    }
}

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
    let mut queue: VecDeque<Calculation> = VecDeque::new();
    queue.push_back(Calculation {
        lights: machine.lights.clone(),
        steps: 0,
    });
    loop {
        let calculation = queue
            .pop_front()
            .expect("machine has to exist at least one");
        // println!("checking calculation = {:?}", calculation);

        if calculation.is_ready() {
            println!("Machine: {:?}, needs steps: {}", machine, calculation.steps);
            return calculation.steps;
        }

        for wire in machine.wirings.iter() {
            let new_calculation = calculation.apply_wire(wire);
            queue.push_back(new_calculation);
        }
    }
}

fn create_queue(machines: &mut &Vec<Machine>) -> VecDeque<Calculation> {
    let mut queue: VecDeque<Calculation> = machines
        .iter()
        .enumerate()
        .map(|(index, machine)| Calculation {
            lights: machine.lights.clone(),
            steps: 0,
        })
        .collect();
    queue
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

            let voltages: Vec<usize> = voltages[1..voltages.len() - 1]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            let voltages = Voltages { values: voltages };

            Machine {
                lights,
                wirings,
                voltages,
            }
        })
        .collect()
}
