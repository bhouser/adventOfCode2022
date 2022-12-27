use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::VecDeque,
};

fn print_stacks(stacks: &Vec<VecDeque<char>>) {
    println!("### stacks ###");
    for stack in stacks.iter() {
        println!("{:?}", stack);
    }
}

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/5.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();


    let mut stacks = Vec::new();

    let mut parse_columns =  true;
    for line in lines {
        let line = line.expect("line was not ok");

        if line.len() == 0 {
            parse_columns = false;
            continue;
        }

        if parse_columns {
            if line.chars().nth(0).expect("char should exist") == ' ' {
                println!("skipping non-input line");
                continue;
            }
            for (i, ch) in line.char_indices() {
                if ch != '[' && ch != ']' && ch != ' ' {
                    let column = i / 4;

                    // println!("Found '{}' at index {}, column {}", ch, i, column);

                    while stacks.len() < column + 1 {
                        stacks.push(VecDeque::new());
                    }

                    stacks[column].push_back(ch);
                }
            }

            println!("building stacks...");
            print_stacks(&stacks);
        } else {
            let split_line: Vec<&str> = line.split(' ').collect();
            let qty: usize = split_line[1].parse().expect("should bed a number");
            let from: usize = split_line[3].parse().expect("should bed a number");
            let to: usize = split_line[5].parse().expect("should bed a number");

            let buf: Vec<char> = stacks[from - 1].drain(0..qty).collect();
            for ch in buf.iter().rev() {
                stacks[to - 1].push_front(*ch);
            }

            println!("moving stuff ...");
            print_stacks(&stacks);
        }
    }

    let mut result = String::new();
    for stack in stacks.iter_mut() {
        let popped = stack.pop_front();
        if let Some(ch) = popped {
            result.push(ch);
        }
    }

    println!("the result: {}", result);
}
