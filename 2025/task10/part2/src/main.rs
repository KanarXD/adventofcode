use std::collections::BTreeMap;
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
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Wiring {
    states: Vec<usize>,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Voltages {
    desired_values: Rc<Vec<usize>>,
    values: Vec<usize>,
}

impl Voltages {
    pub fn score(&self) -> usize {
        self.values.iter().sum()
    }
    fn is_desired(&self) -> VoltageLevel {
        let mut not_enough = false;
        for (desired, power) in self.desired_values.iter().zip(self.values.iter()) {
            if desired < power {
                return VoltageLevel::Overload;
            } else if desired > power {
                not_enough = true
            }
        }
        if not_enough {
            VoltageLevel::Less
        } else {
            VoltageLevel::Equal
        }
    }

    fn apply_wiring(&self, wiring: &Wiring) -> Voltages {
        let mut new_values = self.values.clone();
        for &state in wiring.states.iter() {
            new_values[state] += 1;
        }

        Voltages {
            desired_values: self.desired_values.clone(),
            values: new_values,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum VoltageLevel {
    Less,
    Equal,
    Overload,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Calculation {
    voltages: Voltages,
    steps: u64,
}
impl Calculation {
    fn apply_wire(&self, wire: &Wiring) -> Calculation {
        Calculation {
            steps: self.steps + 1,
            voltages: self.voltages.apply_wiring(wire),
        }
    }

    fn score(&self) -> usize {
        self.voltages.score()
    }
}

fn main() {
    let file_path = "res/demo_input.txt";
    // let file_path = "res/input.txt";

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

fn check_machine2(machine: &Machine) -> u64 {
    let calculation = Calculation {
        steps: 0,
        voltages: machine.voltages.clone(),
    };
    let result = dfs(machine, calculation).expect("result has to exist");

    println!("Machine: {:?}, needs steps: {}", machine, result);
    result
}

fn dfs(machine: &Machine, calculation: Calculation) -> Option<u64> {
    let mut min_calculation = None;
    for wire in machine.wirings.iter() {
        let new_calculation = calculation.apply_wire(&wire);
        match new_calculation.voltages.is_desired() {
            VoltageLevel::Less => {
                if let Some(result) = dfs(machine, new_calculation) {
                    if let Some(min_calculation) = &mut min_calculation {
                        if *min_calculation > result {
                            *min_calculation = result;
                        }
                    } else {
                        min_calculation = Some(result);
                    }
                }
            }
            VoltageLevel::Equal => {
                let result = new_calculation.steps;
                if let Some(min_calculation) = &mut min_calculation {
                    if *min_calculation > result {
                        *min_calculation = result;
                    }
                } else {
                    min_calculation = Some(result);
                }
            }
            VoltageLevel::Overload => {
                // println!("Machine: {:?}, overload: {:?}", machine, new_calculation);
            }
        }
    }
    min_calculation
}

fn check_machine(machine: &Machine) -> u64 {
    let mut queue: BTreeMap<usize, Calculation> = BTreeMap::new();
    // let mut queue: VecDeque<Calculation> = VecDeque::new();
    let calculation = Calculation {
        steps: 0,
        voltages: machine.voltages.clone(),
    };
    queue.insert(calculation.score(), calculation);

    loop {
        let (_, calculation) = queue.pop_last().expect("machine has to exist at least one");
        println!("queue={:?}", queue);

        // println!("checking calculation = {:?}", calculation);

        for wire in machine.wirings.iter() {
            let new_calculation = calculation.apply_wire(&wire);
            match new_calculation.voltages.is_desired() {
                VoltageLevel::Less => {
                    queue.insert(new_calculation.score(), new_calculation);
                }
                VoltageLevel::Equal => {
                    println!(
                        "Machine: {:?}, needs steps: {}",
                        machine, new_calculation.steps
                    );
                    return new_calculation.steps;
                }
                VoltageLevel::Overload => {
                    // println!("Machine: {:?}, overload: {:?}", machine, new_calculation);
                }
            }
        }
    }
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

            let actual_voltages = voltages.iter().map(|_| 0).collect::<Vec<usize>>();

            let voltages = Voltages {
                desired_values: Rc::new(voltages),
                values: actual_voltages,
            };

            Machine {
                lights,
                wirings,
                voltages,
            }
        })
        .collect()
}
