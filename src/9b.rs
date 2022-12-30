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

#[derive(Debug, Clone)]
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

    const TAIL_COUNT: usize = 9;

    let mut tail_offsets = vec![
        head_offset.clone();
        TAIL_COUNT
    ];

    tail_positions.insert(tail_offsets[TAIL_COUNT - 1].position_hash());

    let mut update_tail_offset = |coord: Coord, i| {
        if i == TAIL_COUNT - 1 {
            println!("adding position hash {} for index {}", coord.position_hash(), i);
            tail_positions.insert(coord.position_hash());
        }

        coord
    };

    for line in lines {

        let line = line.expect("line should not error");

        let split_input: Vec<&str> = line.split(" ").collect();
        let direction: Direction = split_input[0].into();
        let distance = split_input[1].parse::<u32>().expect("input should contain number");

        println!("doing {:?} for {}", direction, distance);

        fn new_position(leader: &Coord, follower: &Coord) -> Coord {
            let dx = leader.x - follower.x;
            let dy = leader.y - follower.y;
            let mut x_mov = 0;
            let mut y_mov = 0;

            if dx.abs() > 1 && dy.abs() > 1 {
                x_mov = dx / dx.abs();
                y_mov = dy / dy.abs();
            } else {
                if dx.abs() > 1 {
                    x_mov = dx / dx.abs();
                    if dy.abs() > 0 {
                        y_mov = dy / dy.abs();
                    }
                }
                if dy.abs() > 1 {
                    y_mov = dy / dy.abs();
                    if dx.abs() > 0 {
                        x_mov = dx / dx.abs();
                    }
                }
            }

            Coord {
                x: follower.x + x_mov,
                y: follower.y + y_mov,
            }
        }

        fn update_tail(head_offset: &Coord, tail_offsets: &mut Vec<Coord>, update_tail_offset: &mut dyn FnMut(Coord, usize) -> Coord) {
            for i in 0..TAIL_COUNT {
                let preceding_knot: &Coord = if i == 0 {
                    &head_offset
                } else {
                    &tail_offsets[i-1]
                };

                tail_offsets[i] = update_tail_offset(new_position(&preceding_knot, &tail_offsets[i]), i);
            }
        }

        match direction {
            Direction::Right => {
                for _ in 0..distance {
                    head_offset = Coord {
                        x: head_offset.x + 1,
                        ..head_offset
                    };

                    update_tail(&head_offset, &mut tail_offsets, &mut update_tail_offset);
                }
            },
            Direction::Left => {
                for _ in 0..distance {
                    head_offset = Coord {
                        x: head_offset.x - 1,
                        ..head_offset
                    };

                    update_tail(&head_offset, &mut tail_offsets, &mut update_tail_offset);
                }
            },
            Direction::Down => {
                for _ in 0..distance {
                    head_offset = Coord {
                        y: head_offset.y + 1,
                        ..head_offset
                    };

                    update_tail(&head_offset, &mut tail_offsets, &mut update_tail_offset);
                }
            },
            Direction::Up => {
                for _ in 0..distance {
                    head_offset = Coord {
                        y: head_offset.y - 1,
                        ..head_offset
                    };

                    update_tail(&head_offset, &mut tail_offsets, &mut update_tail_offset);
                }
            },
        }

        println!("head_offset, {:?}", head_offset);
        for i in 0..TAIL_COUNT {
            println!("### tail_offset {}, {:?}", i, tail_offsets[i]);
        }
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
