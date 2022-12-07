use std::cmp::{Eq, PartialEq, Ord,Ordering};

#[derive(Clone)]
pub enum Play {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Play {
    pub fn from_theirs(c: &str) -> Play {
        match c {
            "A" => Play::ROCK,
            "B" => Play::PAPER,
            "C" => Play::SCISSORS,
            _ => panic!("Invalid play for their team: {}", c),
        }
    }

    pub fn from_ours(c: &str) -> Play {
        match c {
            "X" => Play::ROCK,
            "Y" => Play::SCISSORS,
            "Z" => Play::PAPER,
            _ => panic!("Invalid play for our team: {}", c),
        }
    }

    pub fn from_outcome_against(play: &Play, c: &str) -> Play {
        let desired_outcome = match c {
            "X" => Ordering::Less,
            "Y" => Ordering::Equal,
            "Z" => Ordering::Greater,
            _ => panic!("Invalid outcome for our team: {}", c),
        };

        play.determine_play_for_outcome(desired_outcome)
    }

    pub fn get_value(&self) -> i64 {
        match self {
            Play::ROCK => 1,
            Play::PAPER => 2,
            Play::SCISSORS => 3,
        }
    }

    // Returns the opposing Play which will result in the given outcome
    // such that when that play challenges this one, that will be the
    // outcome
    pub fn determine_play_for_outcome(&self, ord: Ordering) -> Play {
        match self {
            Play::ROCK => match ord {
                Ordering::Equal => Play::ROCK,
                Ordering::Greater => Play::SCISSORS,
                Ordering::Less => Play::PAPER,
            },
            Play::PAPER => match ord {
                Ordering::Equal => Play::PAPER,
                Ordering::Greater => Play::SCISSORS,
                Ordering::Less => Play::ROCK,
            },
            Play::SCISSORS => match ord {
                Ordering::Equal => Play::SCISSORS,
                Ordering::Greater => Play::ROCK,
                Ordering::Less => Play::PAPER,
            },
        }
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for Play {}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Play::ROCK => match other {
                Play::ROCK => Ordering::Equal,
                Play::PAPER => Ordering::Less,
                Play::SCISSORS => Ordering::Greater,
            },
            Play::PAPER => match other {
                Play::ROCK => Ordering::Greater,
                Play::PAPER => Ordering::Equal,
                Play::SCISSORS => Ordering::Less,
            },
            Play::SCISSORS => match other {
                Play::ROCK => Ordering::Less,
                Play::PAPER => Ordering::Greater,
                Play::SCISSORS => Ordering::Equal,
            },
        }
    }
}
