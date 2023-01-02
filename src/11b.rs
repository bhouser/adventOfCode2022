use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::{VecDeque, HashMap}
};

#[derive(Debug, Clone, Copy)]
enum OpType {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op_type: OpType,
    op_val: OpVal,
    test_val: usize,
    true_throw: usize,
    false_throw: usize,
    items_inspected: usize,
}

#[derive(Debug, Clone, Copy)]
enum OpVal {
    Old,
    Num(usize),
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

            let items: VecDeque<usize> = items_line.split("Starting items: ")
                .collect::<Vec<&str>>()[1]
                .split(", ").collect::<Vec<&str>>()
                .iter().map(|number_string| number_string.parse::<usize>().expect("should be a number")).collect();

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
                other => OpVal::Num(other.parse::<usize>().expect("operation value should be a number"))
            };

            let test_line = lines.next().expect("line exists").expect("without errors");
            let test_val = test_line.split("Test: divisible by ")
                .collect::<Vec<&str>>()[1].parse::<usize>().expect("test val should be a number");

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
                items_inspected: 0
            });

            _ = lines.next(); // burn one
        } else {
            break;
        }
    }

    let mut next_item_id = 0;

    let monkey_len = monkeys.len();
    let mut item_remainders_map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut monkeys: Vec<Monkey> = monkeys.iter_mut().map(|monkey| {
        let items = monkey.items.iter().map(|item_val| {
            let monkey_item_histories = vec![*item_val; monkey_len];

            let item_id = next_item_id;
            item_remainders_map.insert(item_id, monkey_item_histories); // it needs to be a map :/
            next_item_id += 1;
            item_id
        }).collect();

        Monkey {
            items,
            ..*monkey
        }
    }).collect();

    for game_iteration in 0..10000 {
        println!("game iteration: {}", game_iteration);

        for monkey_index in 0..monkeys.len() {
            loop {
                let item = monkeys[monkey_index].items.pop_front();
                if let Some(item) = item {
                    monkeys[monkey_index].items_inspected += 1;
                    let item_remainders = item_remainders_map.get_mut(&item).expect("item should exist");

                    for i in 0..item_remainders.len() {
                        let remainder = item_remainders[i];

                        let worry_level_factor = match monkeys[monkey_index].op_val {
                            OpVal::Old => remainder,
                            OpVal::Num(num) => num,
                        };

                        let divisible_test_val = monkeys[i].test_val;

                        let new_worry_remainder = match monkeys[monkey_index].op_type {
                            OpType::Add => {
                                (remainder + worry_level_factor) % divisible_test_val
                            },
                            OpType::Multiply => {
                                (remainder * worry_level_factor) % divisible_test_val
                            },
                        };

                        item_remainders[i] = new_worry_remainder;
                    }

                    let receiver_index = if item_remainders[monkey_index] == 0 {
                        monkeys[monkey_index].true_throw
                    } else {
                        monkeys[monkey_index].false_throw
                    };

                    monkeys[receiver_index].items.push_back(item);

                } else {
                    break;
                }
            }
        }
    }

    monkeys.sort_by_key(|m| m.items_inspected);

    let result = monkeys[monkeys.len() - 1].items_inspected * monkeys[monkeys.len() - 2].items_inspected;
    println!("result: {}", result);

    for monkey in monkeys {
        println!("{:?}", monkey);
    }
}
