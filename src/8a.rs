use std::{
    fs::File,
    io::{BufReader, BufRead}
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/8.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    let mut tree_grid = Vec::new();
    let mut visibility_grid = Vec::new();

    let mut cols_len = 0;
    for line in lines {
        let line = line.expect("line was not ok");

        if cols_len == 0 { // the dumbest way I could think of to do this
            cols_len = line.chars().count();
        }

        println!("{}", line);

        let mut new_row = Vec::new();
        for ch in line.chars() {
            let val: i8 = ch.to_string().parse().expect("should be a number");
            new_row.push(val);
        }

        tree_grid.push(new_row);
        visibility_grid.push(vec![0; cols_len]);
    }

    let rows_len = visibility_grid.len();

    // warning: turn back for your own good and sanity
    for i in 0..rows_len {
        let mut tallest_height = -1;
        for j in 0..cols_len {
            if tree_grid[i][j] > tallest_height {
                visibility_grid[i][j] = 1;
                tallest_height = tree_grid[i][j];
            }
            if tallest_height >= 9 {
                break;
            }
        }

        let mut tallest_height = -1;
        for j in (0..cols_len).rev() {
            if tree_grid[i][j] > tallest_height {
                visibility_grid[i][j] = 1;
                tallest_height = tree_grid[i][j];
            }
            if tallest_height >= 9 {
                break;
            }
        }
    }

    for j in 0..cols_len {
        let mut tallest_height = -1;
        for i in 0..rows_len {
            if tree_grid[i][j] > tallest_height {
                visibility_grid[i][j] = 1;
                tallest_height = tree_grid[i][j];
            }
            if tallest_height >= 9 {
                break;
            }
        }

        let mut tallest_height = -1;
        for i in (0..rows_len).rev() {
            if tree_grid[i][j] > tallest_height {
                visibility_grid[i][j] = 1;
                tallest_height = tree_grid[i][j];
            }
            if tallest_height >= 9 {
                break;
            }
        }
    }

    let result = visibility_grid.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, rowVal| acc + rowVal)
    });

    println!("result: {}", result);


}
