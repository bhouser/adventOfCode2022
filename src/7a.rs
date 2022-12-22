use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::{HashMap, VecDeque},
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/7.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    #[derive(Debug)]
    struct FileItem {
        name: String,
        size: i32,
    }

    #[derive(Debug)]
    struct Dir {
        name: String,
        child_dirs: HashMap<String, Dir>,
        child_files: HashMap<String, FileItem>,
    }

    impl Dir {
        fn size(&self) -> i32 {
            let mut size = 0;
            for (_, file) in self.child_files.iter() {
                size += file.size;
            }

            for (_, dir) in self.child_dirs.iter() {
                size += dir.size();
            }

            size
        }
    }

    let root_dir = Dir {
        name: String::from("root"),
        child_dirs: HashMap::new(),
        child_files: HashMap::new(),
    };

    let mut path_history = Vec::new();

    // TODO there will just be one pointer,
    path_history.push(&root_dir);




    // TODO read this: https://www.reddit.com/r/rust/comments/cnjhup/idiomatic_way_to_reference_parent_struct/
    // TODO maybe look at this: https://rust-unofficial.github.io/too-many-lists/

//    let mut score = 0;
    for line in lines {
        if let Ok(l) = line {
            let mut tokens: VecDeque<&str> = l.split(" ").collect();

            println!("tokens: {:?}", tokens);

            let first_token = tokens.pop_front().expect("first token should exist");

            if first_token.eq("$") {
                // it's a command
                let command_token = tokens.pop_front().expect("command token should exist");
                if command_token.eq("cd") {
                    // it's a "cd"
                    let dest_token = tokens.pop_front().expect("cd destination token should exist");

                    // TODO ignore this command
                    if dest_token.eq("/") {
                        // do nothing
                    } else if dest_token.eq("..") {
                        // ah, this is where I need the back links
                    } else {
                        // move filesystem pointer to dir.  Dir must exist.

                        println!("cd into this dir: {}", dest_token);
                        let mut current_location: &mut Dir = *path_history.last().expect("path_history should not be empty");

                        path_history.push(
                                current_location.child_dirs.get_mut(dest_token).expect("dir should exist (have been discovered) before we cd into it")
                        );

//                        filesystem_pointer = filesystem_pointer.child_dirs.get_mut(dest_token).expect("dir should exist (have been discovered) before we cd into it");
                    }
                } // else: it's an "ls".  Do nothing.
            } else if first_token.eq("dir") {
                let new_dir_name_token = tokens.pop_front().expect("dir name token should exist");

                let current_location = path_history.last().expect("path_history should not be empty");
                let existing_dir_option = current_location.child_dirs.get(new_dir_name_token);
                if let None = existing_dir_option {
                    let new_dir = Dir {
                        name: new_dir_name_token.into(),
                        child_dirs: HashMap::new(),
                        child_files: HashMap::new(),
                    };

                    current_location.child_dirs.insert(new_dir_name_token.into(), new_dir);
                }
            } else {
                // it's a new file, so create one
                let new_file_size = first_token.parse().expect("file size token should parse to a number");
                let new_file_name = tokens.pop_front().expect("new file name token should exist");

                let new_file = FileItem {
                    name: new_file_name.into(),
                    size: new_file_size
                };
                let current_location = path_history.last().expect("path_history should not be empty");
                current_location.child_files.entry(new_file_name.into()).or_insert(new_file);
            }
        } else {
            panic!("line was not Ok");
        }
    }

    println!("This is our filesystem tree {:?}", root_dir);

//    println!("total score: {}", score);
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


//        let a_string = "asdfgg";
//
//        let len = a_string.len() / 2;
//
//        assert_eq!(len, 4);
    }
}
