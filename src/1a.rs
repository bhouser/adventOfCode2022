use std::{
    fs::File,
    io::{BufReader, BufRead},
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/1.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    let mut elves = Vec::new();

    let mut sum = 0;
    for line in lines {
        if let Ok(l) = line {
            if l.eq("") {
                println!("empty line");
                elves.push(sum);
                sum = 0;
            } else {
                println!("the line: {}", l);
                sum += l.parse::<i32>().expect("string should be a number");
            }
        } else {
            panic!("line was not Ok");
        }
    }

    elves.push(sum);
    sum = 0;

    println!("current sum: {}", sum);

    println!("the vector: {:?}", elves);

    elves.sort();

    println!("the vector: {:?}", elves);

    println!("largest calorie count: {}", elves.pop().expect("elves should pop 1 value"));
}
