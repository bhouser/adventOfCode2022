use std::{
    fs::File,
    io::{BufReader, BufRead, Lines}, collections::{HashMap, VecDeque}
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/7.txt";

    let file = File::open(file_path).expect("file should be defined");
    let mut lines = BufReader::new(file).lines();

    #[derive(Debug)]
    struct FileItem {
        name: String,
        size: u64,
    }

    #[derive(Debug)]
    struct Dir {
        name: String,
        child_dirs: HashMap<String, Dir>,
        // TODO we don't actually need the file names and sizes like this. Could just add all sibling files into one number.
        child_files: HashMap<String, FileItem>,
    }

    impl Dir {
        fn size(&self) -> u64 {
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

    let mut root_dir = Dir {
        name: String::from("root"),
        child_dirs: HashMap::new(),
        child_files: HashMap::new(),
    };

    fn add_small_dirs(current_dir: &mut Dir, mut total_small_dir_size: u64) -> u64 {
        let current_dir_size = current_dir.size();

        for dir in current_dir.child_dirs.iter_mut() {
            total_small_dir_size = add_small_dirs(dir.1, total_small_dir_size);
        }

        if current_dir_size <= 100000 {
            total_small_dir_size + current_dir_size
        } else {
            total_small_dir_size
        }
    }

    fn process_lines(lines: &mut Lines<BufReader<File>>, current_dir: &mut Dir) {
        loop {
            let line = lines.next();
            if let Some(line) = line {
                let line = line.expect("line should not error");

                let mut tokens: VecDeque<&str> = line.split(" ").collect();
                let first_token = tokens.pop_front().expect("first token should exist");

                if first_token.eq("$") {
                    // it's a command
                    let command_token = tokens.pop_front().expect("command token should exist");
                    if command_token.eq("cd") {
                        // it's a "cd"
                        let dest_token = tokens.pop_front().expect("cd destination token should exist");

                        if dest_token.eq("/") {
                            // do nothing
                            continue;
                        } else if dest_token.eq("..") {
                            return;
                        } else {
                            // dir must exist
                            // println!("cd into this dir: {}", dest_token);
                            let child_dir = current_dir.child_dirs.get_mut(dest_token)
                                .expect("dir should exist (have been discovered) before we cd into it");
                            process_lines(lines, child_dir);
                        }
                    } else {
                        // it's an "ls" token
                        continue;
                    }
                } else if first_token.eq("dir") {
                    let new_dir_name_token = tokens.pop_front().expect("dir name token should exist");

                    // dir shouldn't exist unless we did "ls" twice in same dir
                    let existing_dir_option = current_dir.child_dirs.get(new_dir_name_token);
                    if let None = existing_dir_option {
                        let new_dir = Dir {
                            name: new_dir_name_token.into(),
                            child_dirs: HashMap::new(),
                            child_files: HashMap::new(),
                        };

                        current_dir.child_dirs.insert(new_dir_name_token.into(), new_dir);
                    }
                    continue;
                } else {
                    // it's a new file, so create one
                    let new_file_size = first_token.parse().expect("file size token should parse to a number");
                    let new_file_name = tokens.pop_front().expect("new file name token should exist");

                    let new_file = FileItem {
                        name: new_file_name.into(),
                        size: new_file_size
                    };
                    current_dir.child_files.entry(new_file_name.into()).or_insert(new_file);
                    continue;
                }
            } else {
                return;
                // end of input, initiate return chain.
            }
        }
    }


    process_lines(&mut lines, &mut root_dir);

    // println!("the directory structure:\n{:?}", root_dir);

    // let result = add_small_dirs(&mut root_dir, 0);

    let available_space = 70000000 - root_dir.size();
    println!("available_space {}", available_space);
    let minimum_delete = 30000000 - available_space;

    fn smallest_dir_larger_than(current_dir: &Dir, minimum_delete: u64, mut current_candidate_size: u64) -> u64 {
        let current_dir_size = current_dir.size();

        if current_dir_size > minimum_delete && current_dir_size < current_candidate_size {
            current_candidate_size = current_dir_size;
        }

        for dir in current_dir.child_dirs.iter() {
            current_candidate_size = smallest_dir_larger_than(dir.1, minimum_delete, current_candidate_size);
        }

        current_candidate_size
    }

    let result = smallest_dir_larger_than(&root_dir, minimum_delete, root_dir.size());

    println!("result {}", result);
}
