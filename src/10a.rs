use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::{HashSet}
};

#[derive(Debug)]
enum Instruction {
    Addx { cycles: u8, arg: i32},
    Noop { cycles: u8 },
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(" ").collect();

        if parts[0].eq("noop") {
            Instruction::Noop { cycles: 0 }
        } else {
            Instruction::Addx { cycles: 0, arg: parts[1].parse::<i32>().expect("string should be a number") }
        }
    }
}

impl Instruction {
    fn is_done(&self) -> bool {
        match self {
            Instruction::Addx { cycles, arg: _ } => cycles > &1,
            Instruction::Noop { cycles } => cycles > &0,
        }
    }

    fn do_cycle(&mut self, register: &mut i32) {
        match self {
            Instruction::Addx { cycles, arg } => {
                if cycles == &1 {
                    *register += *arg;
                    println!("added {}, value: {}", arg, register);
                }
                *cycles += 1;
            },
            Instruction::Noop { cycles } => {
                *cycles += 1;
                println!("did noop cycle")
            },
        }
    }
}

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/10.txt";

    let file = File::open(file_path).expect("file should be defined");
    let mut lines = BufReader::new(file).lines();

    let result_cycles = HashSet::from([20, 60, 100, 140, 180, 220]);

    // TODO add scores in 20, 60, 100, 140, 180, 220th cycles
    let mut result = 0;
    let mut register = 1;

    let first_string: &str = &lines.next().expect("next string should exist").expect("without errors");
    let mut current_op: Instruction = first_string.into();
    for cycle in 1.. {
        println!("cycle: {}", cycle);
        if result_cycles.contains(&cycle) {
            result += cycle * register;
            println!("progress: {}, register: {}", result, register);
        }

        if current_op.is_done() {
            if let Some(next_line) = lines.next() {
                let next_line: &str = &next_line.expect("without errors");
                current_op = next_line.into();
            } else {
                break;
            }
        }

        current_op.do_cycle(&mut register)
    }

    println!("result: {}", result);
}
