use std::{
    convert::TryFrom,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Player {
    P1,
    P2,
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl std::convert::TryFrom<char> for Shape {
    type Error = &'static str;
    fn try_from(shape: char) -> Result<Self, &'static str> {
        match shape {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err("unknown"),
        }
    }
}

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

struct GameResult {
    winner: Option<Player>,
}

impl std::convert::TryFrom<char> for GameResult {
    type Error = &'static str;
    fn try_from(shape: char) -> Result<Self, &'static str> {
        let winner = match shape {
            'X' => Some(Player::P1),
            'Y' => None,
            'Z' => Some(Player::P2),
            _ => return Err("unknown"),
        };
        Ok(GameResult { winner })
    }
}

fn part01(file: File) {
    let reader = BufReader::new(file);
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        let c1 = parts.next().unwrap().chars().next().unwrap();
        let c2 = parts.next().unwrap().chars().next().unwrap();

        let p1 = Shape::try_from(c1).unwrap();
        let p2 = Shape::try_from(c2).unwrap();

        let winner = match (p1, p2) {
            (Shape::Rock, Shape::Rock) => None,
            (Shape::Rock, Shape::Paper) => Some(Player::P2),
            (Shape::Rock, Shape::Scissors) => Some(Player::P1),

            (Shape::Paper, Shape::Rock) => Some(Player::P1),
            (Shape::Paper, Shape::Paper) => None,
            (Shape::Paper, Shape::Scissors) => Some(Player::P2),

            (Shape::Scissors, Shape::Rock) => Some(Player::P2),
            (Shape::Scissors, Shape::Paper) => Some(Player::P1),
            (Shape::Scissors, Shape::Scissors) => None,
        };

        score_p1 += p1.score();
        score_p2 += p2.score();
        match winner {
            None => {
                score_p1 += 3;
                score_p2 += 3;
            }
            Some(Player::P1) => score_p1 += 6,
            Some(Player::P2) => score_p2 += 6,
        }
    }
    println!("{} - {}", score_p1, score_p2);
}

fn part02(file: File) {
    let reader = BufReader::new(file);
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        let c1 = parts.next().unwrap().chars().next().unwrap();
        let c2 = parts.next().unwrap().chars().next().unwrap();

        let p1 = Shape::try_from(c1).unwrap();
        let rs = GameResult::try_from(c2).unwrap();

        let p2 = match (p1, rs.winner.clone()) {
            (Shape::Rock, None) => Shape::Rock,
            (Shape::Rock, Some(Player::P1)) => Shape::Scissors,
            (Shape::Rock, Some(Player::P2)) => Shape::Paper,

            (Shape::Paper, None) => Shape::Paper,
            (Shape::Paper, Some(Player::P1)) => Shape::Rock,
            (Shape::Paper, Some(Player::P2)) => Shape::Scissors,

            (Shape::Scissors, None) => Shape::Scissors,
            (Shape::Scissors, Some(Player::P1)) => Shape::Paper,
            (Shape::Scissors, Some(Player::P2)) => Shape::Rock,
        };

        score_p1 += p1.score();
        score_p2 += p2.score();
        match rs.winner {
            None => {
                score_p1 += 3;
                score_p2 += 3;
            }
            Some(Player::P1) => score_p1 += 6,
            Some(Player::P2) => score_p2 += 6,
        }
    }
    println!("{} - {}", score_p1, score_p2);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    part01(File::open(&fpath).unwrap());
    part02(File::open(&fpath).unwrap());
}
