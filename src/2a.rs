use std::{
    fs::File,
    io::{BufReader, BufRead}, sync::RwLock, collections::HashMap,
};

fn main() {
    println!("Hello, worldddd!");

    let file_path = "data/2.txt";

    let file = File::open(file_path).expect("file should be defined");
    let lines = BufReader::new(file).lines();

    #[derive(PartialEq, Eq)]
    enum Hand {
        rock,
        paper,
        scissors
    }

    let mut handLookup: HashMap<&str, Hand> = HashMap::from([
        ("A", Hand::rock),
        ("B", Hand::paper),
        ("C", Hand::scissors),
        ("X", Hand::rock),
        ("Y", Hand::paper),
        ("Z", Hand::scissors),
    ]);

    let mut score = 0;
    for line in lines {
        if let Ok(l) = line {
            let pair: Vec<&str> = l.split(' ').collect();
            let opponent_letter = handLookup.get(pair[0]);
            let my_letter = handLookup.get(pair[1]);




            if let Some(opponent_hand) = opponent_letter {
                if let Some(my_hand) = my_letter {

                    // match (my_hand, opponent_hand) {
                    //     (Hand::rock, Hand::rock) => score += 1 + 3,
                    //     (Hand::rock, Hand::paper) => score += 1,
                    //     (Hand::rock, Hand::scissors) => score += 1 + 6,
                    //     (Hand::paper, Hand::rock) => score += 2,
                    //     (Hand::paper, Hand::paper) => score += 2,
                    //     (Hand::paper, Hand::scissors) => score += 2,
                    //     (Hand::scissors, Hand::rock) => score += 3,
                    //     (Hand::scissors, Hand::paper) => score += 3,
                    //     (Hand::scissors, Hand::scissors) => score += 3,
                    // }


                    if opponent_hand == my_hand {
                        score += 3;
                    } else {
                        if opponent_hand == &Hand::rock && my_hand == &Hand::paper
                            || opponent_hand == &Hand::paper && my_hand == &Hand::scissors
                            || opponent_hand == &Hand::scissors && my_hand == &Hand::rock {
                                score += 6;
                        }
                    }

                    match my_hand {
                        Hand::rock => score += 1,
                        Hand::paper => score += 2,
                        Hand::scissors => score += 3
                    }
                }
            }
        } else {
            panic!("line was not Ok");
        }
    }

    println!("total score: {}", score);



}
