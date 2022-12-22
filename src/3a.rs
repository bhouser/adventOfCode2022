use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::{HashSet, HashMap},
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/3.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();


    let char_value_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    .chars()
    .zip(1..53)
    .collect::<HashMap<char, usize>>();

    let mut score = 0;
    for line in lines {
        if let Ok(l) = line {
            let mut right = HashSet::new();

            // TODO get a slice of half the length
            let half_index = l.len() / 2;
            let left_string_half = &l[..half_index];
            let right_string_half = &l[half_index..];

            for character in right_string_half.chars() {
                right.insert(character);
            }

            for character in left_string_half.chars() {
                if let Some(&val) = right.get(&character) {
                    if let Some(priority) = char_value_map.get(&val) {
                        score += priority;
                    }
                    break;
                }
            }
        } else {
            panic!("line was not Ok");
        }
    }

    println!("total score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let output = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..53)
        .collect::<HashMap<char, usize>>();

        println!("this is the output {:?}", output);
    }
}
