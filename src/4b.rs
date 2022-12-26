use std::{
    fs::File,
    io::{BufReader, BufRead},
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/4.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    struct Pair {
        a_start: isize,
        a_end: isize,
        b_start: isize,
        b_end: isize,
    }

    impl Pair {
        fn fully_contains(&self) -> bool {
            self.a_start <= self.b_end && self.a_end >= self.b_start
        }
    }

    let result: isize = lines.map(|line_result| {
        line_result.expect("line should exist")
    }).map(|line| {

        let parts: Vec<&str> = line.split(',').collect();
        let a: Vec<&str> = parts[0].split('-').collect();
        let b: Vec<&str> = parts[1].split('-').collect();

        Pair {
            a_start: a[0].parse::<isize>().expect("a0 string should be a number"),
            a_end: a[1].parse::<isize>().expect("a1 string should be a number"),
            b_start: b[0].parse::<isize>().expect("b0 string should be a number"),
            b_end: b[1].parse::<isize>().expect("b1 string should be a number"),
        }
    }).map(|pair| pair.fully_contains()).fold(0, |acc, x| {
        if x {
            acc + 1
        } else {
            acc
        }
    });

    println!("total number of pairs with one range fully containing the other: {}", result);
}
