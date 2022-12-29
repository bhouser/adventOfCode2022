use std::{
    fs::File,
    io::{BufReader, BufRead},
    cmp,
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


    fn compute_scenic_score(row: usize, col: usize, tree_grid: &Vec<Vec<i8>>) -> u32 {
        let cols_len = tree_grid.len();
        let rows_len = tree_grid[0].len();
        let height = tree_grid[row][col];

        let mut right = 0;
        let mut left = 0;
        let mut down = 0;
        let mut up = 0;
        for i in col + 1..cols_len {
            // does this execute if the range is out of bounds?  It doesn't!!
            if tree_grid[row][i] <= height {
                right += 1;
            }
            if tree_grid[row][i] == height {
                break;
            }
        }

        for i in (0..col).rev() {
            // does this execute if the range is out of bounds?  It doesn't!!
            if tree_grid[row][i] <= height {
                left += 1;
            }
            if tree_grid[row][i] == height {
                break;
            }
        }

        for j in row + 1..rows_len {
            // does this execute if the range is out of bounds?  It doesn't!!
            if tree_grid[j][col] <= height {
                down += 1;
            }
            if tree_grid[j][col] == height {
                break;
            }
        }

        for j in (0..row).rev() {
            // does this execute if the range is out of bounds?  It doesn't!!
            if tree_grid[j][col] <= height {
                up += 1;
            }
            if tree_grid[j][col] == height {
                break;
            }
        }

        let scenic_score = right * left * down * up;

        if (scenic_score > 10000) {
            println!("score: {}, right: {}, left: {}, down: {}, up: {}", scenic_score, right, left, down, up);
        }
        // println!("scenic score: {}", scenic_score);

        // right * left * down * up
        scenic_score
    }

    // let result = visibility_grid.iter().fold(0, |acc, row| {
    //     acc + row.iter().fold(0, |acc, rowVal| acc + rowVal)
    // });

    let result = visibility_grid.iter().enumerate().fold(0, |acc, (j, row)| {
        let most_scenic_in_row = row.iter().enumerate().map(|(i, _)| compute_scenic_score(j,i, &tree_grid))
            .fold(0, |acc, scenic_score| cmp::max(acc, scenic_score));

        println!("most scenic in row {}", most_scenic_in_row);
        cmp::max(acc, most_scenic_in_row)
    });


    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("hi");

        for i in 6..5 {
            println!("i: {}", i);
        }
    }
}
