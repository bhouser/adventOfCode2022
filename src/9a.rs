use std::{
    fs::File,
    io::{BufReader, BufRead},
    collections::HashSet, cmp,
};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from (s: &str) -> Self {
        match s {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "U" => Direction::Up,
            _ => panic!("not a valid direction")
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn position_hash(&self) -> String {
        format!("{}-{}", self.x, self.y)
    }
}

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/9.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    let mut tail_positions = HashSet::new();

    let mut head_offset = Coord {
        x: 0,
        y: 0
    };

    
    let mut tail_offset = Coord {
        ..head_offset
    };
    tail_positions.insert(tail_offset.position_hash());

    let mut update_tail_offset = |x, y| {
        let new_coord = Coord{
            x,
            y,
        };

        println!("adding position hash {}", new_coord.position_hash());
        tail_positions.insert(new_coord.position_hash());
        new_coord
    };

    for line in lines {

        let line = line.expect("line should not error");

        let split_input: Vec<&str> = line.split(" ").collect();
        let direction: Direction = split_input[0].into();
        let distance = split_input[1].parse::<u32>().expect("input should contain number");

        println!("doing {:?} for {}", direction, distance);

        match direction {
            Direction::Right => {
                for _ in 0..distance {
                    head_offset = Coord {
                        x: head_offset.x + 1,
                        ..head_offset
                    };
                    
                    if head_offset.x - tail_offset.x > 1 {
                        if tail_offset.y == head_offset.y {
                            tail_offset = update_tail_offset(tail_offset.x + 1, tail_offset.y);
                        } else {
                            tail_offset = update_tail_offset(tail_offset.x + 1,  tail_offset.y + head_offset.y - tail_offset.y);
                        }
                    }
                }
            },
            Direction::Left => {
                for _ in 0..distance {
                    head_offset = Coord {
                        x: head_offset.x - 1,
                        ..head_offset
                    };
                    
                    if tail_offset.x - head_offset.x > 1 {
                        if tail_offset.y == head_offset.y {
                            tail_offset = update_tail_offset(tail_offset.x - 1, tail_offset.y);
                        } else {
                            tail_offset = update_tail_offset(tail_offset.x - 1,  tail_offset.y + head_offset.y - tail_offset.y);
                        }
                    }
                }
            },
            Direction::Down => {
                for _ in 0..distance {
                    head_offset = Coord {
                        y: head_offset.y + 1,
                        ..head_offset
                    };
                    
                    if head_offset.y - tail_offset.y > 1 {
                        if tail_offset.x == head_offset.x {
                            tail_offset = update_tail_offset(tail_offset.x, tail_offset.y + 1);
                        } else {
                            tail_offset = update_tail_offset(tail_offset.x + head_offset.x - tail_offset.x,  tail_offset.y + 1);
                        }
                    }
                }
            },
            Direction::Up => {
                for _ in 0..distance {
                    head_offset = Coord {
                        y: head_offset.y - 1,
                        ..head_offset
                    };
                    
                    if tail_offset.y - head_offset.y > 1 {
                        if tail_offset.x == head_offset.x {
                            tail_offset = update_tail_offset(tail_offset.x, tail_offset.y - 1);
                        } else {
                            tail_offset = update_tail_offset(tail_offset.x + head_offset.x - tail_offset.x,  tail_offset.y - 1);
                        }
                    }
                }
            },
        }

        println!("head_offset, {:?}", head_offset);
        println!("tail_offset, {:?}", tail_offset);
    }

    let result = tail_positions.iter().count();
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let my_coord = Coord {
            x: -7,
            y: -7
        };

        println!("coord_hash: {:?}", my_coord.position_hash());
    }
}
