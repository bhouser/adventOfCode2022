use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::HashSet,
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/6.txt";

    let file = File::open(file_path).expect("file should be defined");
    let line = BufReader::new(file).lines().next().expect("line should exist").expect("without error");

    let unique_char_sequence_len = 14;
    for ch in line.char_indices() {
        if ch.0 < unique_char_sequence_len {
            continue;
        }
        let substring = &line[(ch.0 - unique_char_sequence_len)..ch.0];
        let char_set: HashSet<char> = substring.chars().collect();
        if char_set.len() == unique_char_sequence_len {
            println!("index until unique 14: {}", ch.0);
            break;
        }
    }
}
