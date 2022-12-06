use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::{HashSet},
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/3.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();



    let mut score = 0;
    for line in lines {
        if let Ok(l) = line {
            //  TODO find the item which is duplicated across compartments
            // just make a set? of each side, then iterate the set, and check if the other set has it.

            // also: computing priorities: compute a charcode offset?  Alternatively: specify each value?


            let mut left: HashSet<&str> = HashSet::new();
            let mut right: HashSet<&str> = HashSet::new();

            // TODO get a slice of half the length
            let half_index = l.len() / 2;
            let left_string_half = &l[..half_index];
            let right_string_half = &l[half_index..];

            for character in left_string_half.split("") {
                left.insert(character);
            }

            println!("leftSet: {:?}", left);

            for character in right_string_half.split("") {
                right.insert(character);
            }

            println!("rightSet: {:?}", right);

            for character in left {
                if let Some(&val) = right.get(character) {
                    println!("Found the duplicate: {}", val);
                    break;
                }
            }

            score += 1;
            if score == 3 {
                break;
            }
        } else {
            panic!("line was not Ok");
        }
    }

    println!("total score: {}", score);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a_string = "asdfgg";

        let len = a_string.len() / 2;

        assert_eq!(len, 4);
    }
}
