use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn distance(&self, other: &Self) -> u64 {
        let xdiff = (self.x - other.x).abs() as u64;
        let ydiff = (self.y - other.y).abs() as u64;
        xdiff.max(ydiff)
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn new(len: usize) -> Self {
        Self {
            knots: vec![Position::default(); len],
        }
    }

    fn step(&mut self, direction: &Direction) {
        if let Some(head) = self.knots.first_mut() {
            head.step(direction);
        }

        let knots_count = self.knots.len();
        for i in 1..knots_count {
            if self.knots[i - 1].distance(&self.knots[i]) > 1 {
                if self.knots[i - 1].x > self.knots[i].x {
                    self.knots[i].step(&Direction::Right);
                } else if self.knots[i - 1].x < self.knots[i].x {
                    self.knots[i].step(&Direction::Left);
                };
                if self.knots[i - 1].y > self.knots[i].y {
                    self.knots[i].step(&Direction::Up);
                } else if self.knots[i - 1].y < self.knots[i].y {
                    self.knots[i].step(&Direction::Down);
                };
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = std::io::Error;

    fn try_from(s: &str) -> std::io::Result<Self> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "unknown direction",
            )),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u64,
}

impl TryFrom<&str> for Instruction {
    type Error = std::io::Error;

    fn try_from(s: &str) -> std::io::Result<Self> {
        let mut tokens = s.split_whitespace();

        let direction = tokens.next().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expecting direction token",
        ))?;
        let direction = Direction::try_from(direction)?;

        let steps = tokens.next().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expecting steps token",
        ))?;
        let steps = steps
            .parse::<u64>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        Ok(Self { direction, steps })
    }
}

fn simulate(rope: &mut Rope, instructions: &[Instruction]) -> usize {
    let mut visited: HashSet<Position> = HashSet::default();
    if let Some(tail) = rope.knots.last() {
        visited.insert(tail.clone());
    }
    for inst in instructions {
        for _ in 0..inst.steps {
            rope.step(&inst.direction);
            if let Some(tail) = rope.knots.last() {
                visited.insert(tail.clone());
            }
        }
    }
    visited.len()
}

fn part01(instructions: &[Instruction]) {
    let mut rope = Rope::new(2);
    let count = simulate(&mut rope, instructions);
    println!("{}", count);
}

fn part02(instructions: &[Instruction]) {
    let mut rope = Rope::new(10);
    let count = simulate(&mut rope, instructions);
    println!("{}", count);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    let reader = BufReader::new(File::open(&fpath).unwrap());
    let instructions: Vec<Instruction> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| Instruction::try_from(line.as_str()))
        .map(Result::unwrap)
        .collect();

    part01(&instructions);
    part02(&instructions);
}
