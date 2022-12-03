use core::panic;

#[derive(PartialEq, Eq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum Result {
    Win,
    Lose,
    Tie,
}

impl Result {
    fn to_usize(&self) -> usize {
        match self {
            Result::Win => 6,
            Result::Tie => 3,
            Result::Lose => 0,
        }
    }
}

impl Hand {
    fn new(input: char) -> Hand {
        match input {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("ahh"),
        }
    }

    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn get_result(&self, other_hand: &Hand) -> Result {
        match self {
            Hand::Rock => match other_hand {
                Hand::Rock => Result::Tie,
                Hand::Paper => Result::Lose,
                Hand::Scissors => Result::Win,
            },
            Hand::Paper => match other_hand {
                Hand::Rock => Result::Win,
                Hand::Paper => Result::Tie,
                Hand::Scissors => Result::Lose,
            },
            Hand::Scissors => match other_hand {
                Hand::Rock => Result::Lose,
                Hand::Paper => Result::Win,
                Hand::Scissors => Result::Tie,
            },
        }
    }

    fn find_hand_for_result(&self, result: &Result) -> Hand {
        match self {
            Hand::Rock => match result {
                Result::Win => Hand::Paper,
                Result::Tie => Hand::Rock,
                Result::Lose => Hand::Scissors,
            },
            Hand::Paper => match result {
                Result::Win => Hand::Scissors,
                Result::Tie => Hand::Paper,
                Result::Lose => Hand::Rock,
            },
            Hand::Scissors => match result {
                Result::Win => Hand::Rock,
                Result::Tie => Hand::Scissors,
                Result::Lose => Hand::Paper,
            },
        }
    }
}

struct Round {
    selff: Hand,
    other: Hand,
}

impl Round {
    fn new_vec(input: &str) -> Vec<Round> {
        let mut rounds = Vec::new();
        for round in input.split('\n') {
            rounds.push(Round::new(round));
        }
        rounds
    }

    fn new(input: &str) -> Round {
        let chars: Vec<char> = input.chars().collect();
        Round {
            selff: Hand::new(chars[2]),
            other: Hand::new(chars[0]),
        }
    }

    fn get_score(&self) -> usize {
        self.selff.get_result(&self.other).to_usize() + self.selff.score()
    }
}

fn solutiona(input: &str) -> usize {
    let rounds = Round::new_vec(input);
    rounds.iter().map(|round| round.get_score()).sum()
}

fn solutionb(input: &str) -> usize {
    let mut opponent_hands = Vec::new();
    let mut wins_loses = Vec::new();

    for round in input.split('\n') {
        let chars: Vec<char> = round.chars().collect();
        opponent_hands.push(Hand::new(chars[0]));
        let win_lose = match chars[2] {
            'X' => Result::Lose,
            'Y' => Result::Tie,
            'Z' => Result::Win,
            _ => panic!(),
        };
        wins_loses.push(win_lose);
    }

    opponent_hands
        .iter()
        .enumerate()
        .map(|(i, elem)| {
            elem.find_hand_for_result(&wins_loses[i]).score() + wins_loses[i].to_usize()
        })
        .sum()
}

fn main() {
    let text = include_str!("day02.txt");
    println!("Day 2a: {}", solutiona(text));
    println!("Day 2b: {}", solutionb(text));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examplea() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(solutiona(input), 15)
    }

    #[test]
    fn exampleb() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(solutionb(input), 12)
    }
}
