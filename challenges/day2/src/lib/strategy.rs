use std::cmp::Ordering;

use super::play::Play;

pub struct Strategy {
    pub theirs: Play,
    pub ours: Play, 
}

impl Strategy {
    pub fn from_part1(line: &String) -> Strategy {
        let tokens: Vec<&str> = line.split(' ').collect();

        if tokens.len() < 2 {
            panic!("Not enough tokens parsed to generate a strategy");
        }

        Strategy {
            theirs: Play::from_theirs(tokens[0]),
            ours: Play::from_ours(tokens[1]),
        }
    }

    pub fn from_part2(line: &String) -> Strategy {
        let tokens: Vec<&str> = line.split(' ').collect();

        if tokens.len() < 2 {
            panic!("Not enough tokens parsed to generate a strategy");
        }

        let theirs = Play::from_theirs(tokens[0]);
        let ours = Play::from_outcome_against(&theirs, tokens[1]);

        Strategy { theirs, ours }
    }

    pub fn validate(line: &String) -> bool {
        let tokens: Vec<&str> = line.split(' ').collect();

        tokens.len() == 2
    }

    pub fn compare(&self) -> Ordering {
        self.ours.cmp(&self.theirs)
    }

    pub fn get_value(&self) -> i64 {
        let play_score = self.ours.get_value();

        let outcome_score = match self.compare() {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        play_score + outcome_score
    }
}
