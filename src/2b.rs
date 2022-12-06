use std::{
    fs::File,
    io::{BufReader, BufRead}, collections::HashMap,
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/2.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    #[derive(PartialEq, Eq)]
    enum Hand {
        Rock = 1,
        Paper = 2,
        Scissors = 3
    }

    impl Hand {
        fn value(&self) -> i32 {
            match *self {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            }
        }
    }

    #[derive(PartialEq, Eq)]
    enum Play {
        Lose = 0,
        Draw = 3,
        Win = 6
    }

    impl Play {
        fn value(&self) -> i32 {
            match *self {
                Play::Lose => 0,
                Play::Draw => 3,
                Play::Win => 6,
            }
        }
    }

    let hand_lookup: HashMap<&str, Hand> = HashMap::from([
        ("A", Hand::Rock),
        ("B", Hand::Paper),
        ("C", Hand::Scissors),
        ]);

    let play_lookup: HashMap<&str, Play> = HashMap::from([
        ("X", Play::Lose),
        ("Y", Play::Draw),
        ("Z", Play::Win),
        ]);

    let mut score = 0;
    for line in lines {
        if let Ok(l) = line {
            let pair: Vec<&str> = l.split(' ').collect();
            let opponent_letter = hand_lookup.get(pair[0]);
            let my_letter = play_lookup.get(pair[1]);

            if let Some(opponent_hand) = opponent_letter {
                if let Some(my_play) = my_letter {
                    score += my_play.value();

                    match my_play {
                        Play::Win => match opponent_hand {
                            Hand::Rock => score += Hand::Paper.value(),
                            Hand::Paper => score += Hand::Scissors.value(),
                            Hand::Scissors => score += Hand::Rock.value(),
                        },
                        Play::Draw => {
                            score += opponent_hand.value()
                        },
                        Play::Lose => match opponent_hand {
                            Hand::Rock => score += Hand::Scissors.value(),
                            Hand::Paper => score += Hand::Rock.value(),
                            Hand::Scissors => score += Hand::Paper.value(),
                        },
                    }
                }
            }
        } else {
            panic!("line was not Ok");
        }
    }

    println!("total score: {}", score);
}
