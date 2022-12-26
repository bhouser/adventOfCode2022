use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::{HashSet, HashMap},
};
use itertools::Itertools;


fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/3.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    let char_value_map: HashMap<char, i32> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    .chars()
    .zip(1..53)
    .collect();

    struct ElfGroup {
        rucksacks: (String, String, String),
    }

    impl ElfGroup {
        fn get_priority(&self, char_value_map: &HashMap<char, i32>) -> i32 {
            let second: HashSet<char> = self.rucksacks.1.chars().collect();
            let third: HashSet<char> = self.rucksacks.2.chars().collect();

            for character in self.rucksacks.0.chars() {
                if let Some(_) = second.get(&character) {
                    if let Some(_) = third.get(&character) {
                        if let Some(priority) = char_value_map.get(&character) {
                            return priority.clone();
                        }
                    }
                }
            }

            // return 0 if no triple-match found.  Can't happen.
            0
        }
    }


    let mut line_strings = Vec::new();
    for line in lines {
        if let Ok(l) = line { 
            line_strings.push(l);
        }
    }

    let mut groups = Vec::new();
    // TODO instead of adding a library to call chunks on an iter, I should have used navtive .chunks on a vec
    for mut chunk in &line_strings.into_iter().chunks(3) {
        let one = chunk.next().expect("first group member should exist");
        let two = chunk.next().expect("second group member should exist");
        let three = chunk.next().expect("third group member should exist");

        let group = ElfGroup {
            

            rucksacks: (one, two, three)
        };
        groups.push(group);
    }

    let total_priority: i32 = groups.into_iter().map(|group| {
        group.get_priority(&char_value_map)
    }).sum();

    println!("total score: {}", total_priority);
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
