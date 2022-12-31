use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::VecDeque
};

#[derive(Debug)]
enum OpType {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    op_type: OpType,
    op_val: OpVal,
    test_val: u128,
    true_throw: usize,
    false_throw: usize,
    items_inspected: u32,
}

#[derive(Debug)]
enum OpVal {
    Old,
    Num(u128),
}

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/11.txt";

    let file = File::open(file_path).expect("file should be defined");
    let mut lines = BufReader::new(file).lines();

    let mut monkeys = Vec::new();
    loop {
        if let Some(_) = lines.next() {
            // ignore first line, monkeys are introduced in order and will be ID'd by their index

            let items_line = lines.next().expect("line exists").expect("without errors");
            println!("items line: {}", items_line);

            let items: VecDeque<u128> = items_line.split("Starting items: ")
                .collect::<Vec<&str>>()[1]
                .split(", ").collect::<Vec<&str>>()
                .iter().map(|number_string| number_string.parse::<u128>().expect("should be a number")).collect();

            println!("items: {:?}", items);

            let op_line = lines.next().expect("line exists").expect("without errors");
            let op_tokens = op_line.split("Operation: new = old ")
                .collect::<Vec<&str>>()[1]
                .split(" ").collect::<Vec<&str>>();

            let op_type = match op_tokens[0] {
                "*" => OpType::Multiply,
                "+" => OpType::Add,
                _ => panic!("unknown operation type")
            };

            let op_val = match op_tokens[1] {
                "old" => OpVal::Old,
                other => OpVal::Num(other.parse::<u128>().expect("operation value should be a number"))
            };

            let test_line = lines.next().expect("line exists").expect("without errors");
            let test_val = test_line.split("Test: divisible by ")
                .collect::<Vec<&str>>()[1].parse::<u128>().expect("test val should be a number");

            let true_throw_line = lines.next().expect("line exists").expect("without errors");
            let true_throw = true_throw_line.split("If true: throw to monkey ")
            .collect::<Vec<&str>>()[1].parse::<usize>().expect("true throw should be a number");

            let false_throw_line = lines.next().expect("line exists").expect("without errors");
            let false_throw = false_throw_line.split("If false: throw to monkey ")
            .collect::<Vec<&str>>()[1].parse::<usize>().expect("false throw should be a number");

            monkeys.push(Monkey {
                items,
                op_type,
                op_val,
                test_val,
                true_throw,
                false_throw,
                items_inspected: 0,
            });

            _ = lines.next(); // burn one
        } else {
            break;
        }
    }

    for game_iteration in 0..20 {
        println!("game iteration: {}", game_iteration);

        for monkey_index in 0..monkeys.len() {
            loop {
                let item = monkeys[monkey_index].items.pop_front();
                if let Some(item) = item {
                    monkeys[monkey_index].items_inspected += 1;
                    let worry_level_factor = match monkeys[monkey_index].op_val {
                        OpVal::Old => item,
                        OpVal::Num(num) => num,
                    };
                    // println!("item {}, factor: {}", item, worry_level_factor);
                    let new_worry_level = match monkeys[monkey_index].op_type {
                        OpType::Add => item + worry_level_factor,
                        OpType::Multiply => item * worry_level_factor,
                    };

                    let new_worry_level = new_worry_level / 3;

                    let receiver_index = if new_worry_level % monkeys[monkey_index].test_val == 0 {
                        monkeys[monkey_index].true_throw
                    } else {
                        monkeys[monkey_index].false_throw
                    };

                    monkeys[receiver_index].items.push_back(new_worry_level);
                } else {
                    break;
                }
            }
        }
    }

    monkeys.sort_by_key(|m| m.items_inspected);

    let result = monkeys[monkeys.len() - 1].items_inspected * monkeys[monkeys.len() - 2].items_inspected;
    println!("result: {}", result);
}
