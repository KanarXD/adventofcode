use std::fmt::Debug;
use std::fs;
use std::ops::BitXor;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Computer {
    program: Vec<u32>,
    registers: Registers,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct ComputerState {
    computer: Computer,
    pointer: usize,
    move_pointer: bool,
}

impl ComputerState {
    fn invalid_pointer(&self) -> bool {
        self.pointer >= self.computer.program.len()
    }
    fn increase_pointer(&mut self) {
        self.pointer += 2;
    }
    fn opcode(&self) -> u32 {
        self.computer.program[self.pointer]
    }

    fn literal_operand(&self) -> u32 {
        self.computer.program[self.pointer + 1]
    }

    fn combo_operand(&self) -> u32 {
        match self.literal_operand() {
            0..=3 => self.literal_operand(),
            4 => self.computer.registers.a,
            5 => self.computer.registers.b,
            6 => self.computer.registers.c,
            _ => panic!("invalid resolve combo operand"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

struct Instruction {
    opcode: u32,
    action: fn(&mut ComputerState) -> Option<u32>,
}

const INSTRUCTIONS: &[&Instruction] = &[
    &Instruction {
        opcode: 0,
        action: |computer_state| {
            let numerator = computer_state.computer.registers.a;
            let denominator = 2_u32.pow(computer_state.combo_operand());
            computer_state.computer.registers.a = numerator / denominator;
            None
        },
    },
    &Instruction {
        opcode: 1,
        action: |computer_state| {
            computer_state.computer.registers.b = computer_state
                .computer
                .registers
                .b
                .bitxor(computer_state.literal_operand());
            None
        },
    },
    &Instruction {
        opcode: 2,
        action: |computer_state| {
            computer_state.computer.registers.b = computer_state.combo_operand() % 8;
            None
        },
    },
    &Instruction {
        opcode: 3,
        action: |computer_state| {
            if computer_state.computer.registers.a == 0 {
                return None;
            }
            computer_state.move_pointer = false;
            computer_state.pointer = computer_state.literal_operand() as usize;
            None
        },
    },
    &Instruction {
        opcode: 4,
        action: |computer_state| {
            computer_state.computer.registers.b = computer_state
                .computer
                .registers
                .b
                .bitxor(computer_state.computer.registers.c);
            None
        },
    },
    &Instruction {
        opcode: 5,
        action: |computer_state| Some(computer_state.combo_operand() % 8),
    },
    &Instruction {
        opcode: 6,
        action: |computer_state| {
            let numerator = computer_state.computer.registers.a;
            let denominator = 2_u32.pow(computer_state.combo_operand());
            computer_state.computer.registers.b = numerator / denominator;
            None
        },
    },
    &Instruction {
        opcode: 7,
        action: |computer_state| {
            let numerator = computer_state.computer.registers.a;
            let denominator = 2_u32.pow(computer_state.combo_operand());
            computer_state.computer.registers.c = numerator / denominator;
            None
        },
    },
];

fn main() {
    let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let computer: Computer = parse_lines(data);
    println!("computer={:?}", computer);

    let output = process_data(&computer);
    println!("output={}", output)
}

fn parse_lines(data: String) -> Computer {
    let (registers_string, program_string) = data.split_once("\n\n").unwrap();
    let register_values: Vec<u32> = registers_string
        .split("\n")
        .enumerate()
        .map(|(i, register_string)| {
            let (_, value_string) = register_string.split_once(": ").unwrap();
            let value = value_string.parse::<u32>().unwrap();

            return value;
        })
        .collect();
    let registers = Registers {
        a: register_values[0],
        b: register_values[1],
        c: register_values[2],
    };
    let program: Vec<u32> = program_string
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|instruction_string| instruction_string.parse().unwrap())
        .collect();

    Computer { program, registers }
}

fn process_data(computer: &Computer) -> String {
    let mut computer_state = ComputerState {
        computer: computer.clone(),
        pointer: 0,
        move_pointer: true,
    };

    let mut output_values: Vec<u32> = vec![];

    loop {
        let output = run_command(&mut computer_state);

        if let Some(num) = output {
            output_values.push(num)
        }

        if computer_state.move_pointer {
            computer_state.increase_pointer();
        }
        computer_state.move_pointer = true;

        if computer_state.invalid_pointer() {
            break;
        }
    }

    println!("final computer {computer_state:?}");

    output_values
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn run_command(mut computer_state: &mut ComputerState) -> Option<u32> {
    let instruction = INSTRUCTIONS
        .into_iter()
        .find(|&instruction| instruction.opcode == computer_state.opcode())
        .unwrap();
    (instruction.action)(&mut computer_state)
}
