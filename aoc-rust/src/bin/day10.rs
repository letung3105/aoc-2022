use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
enum Instruction {
    ADDX(i32),
    NOOP,
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::ADDX(_) => 2,
            Self::NOOP => 1,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = std::io::Error;

    fn try_from(s: &str) -> std::io::Result<Self> {
        let mut tokens = s.trim().split_whitespace();
        let instruction_name = tokens.next().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expecting instruction name",
        ))?;
        match instruction_name {
            "addx" => {
                let val = tokens
                    .next()
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "expecting an integer",
                    ))?
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
                Ok(Self::ADDX(val))
            }
            "noop" => Ok(Self::NOOP),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "unknown instruction",
            )),
        }
    }
}

#[derive(Debug)]
struct CPU {
    cycles_processing: usize,
    program: Vec<Instruction>,
    program_counter: usize,
    register_x: i32,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            cycles_processing: usize::default(),
            program: Vec::default(),
            program_counter: usize::default(),
            register_x: 1,
        }
    }
}

impl CPU {
    fn load(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.program_counter = 0;
    }

    fn start(&mut self) {
        self.cycles_processing += 1;
    }

    fn stop(&mut self) {
        if let Some(inst) = self.program.get(self.program_counter) {
            if self.cycles_processing == inst.cycles() {
                match inst {
                    Instruction::NOOP => {}
                    Instruction::ADDX(n) => self.register_x += n,
                }
                self.cycles_processing = 0;
                self.program_counter += 1;
            }
        }
    }
}

fn part01(instructions: Vec<Instruction>) {
    let mut cpu = CPU::default();
    cpu.load(instructions);

    let mut cycles: HashSet<usize> = HashSet::default();
    cycles.insert(20);
    for i in (60..=220).step_by(40) {
        cycles.insert(i);
    }

    let mut sum_signal_strength = 0;
    for cycle in 1..=220 {
        cpu.start();
        if cycles.contains(&cycle) {
            sum_signal_strength += cycle as i32 * cpu.register_x;
        }
        cpu.stop();
    }
    println!("{}", sum_signal_strength);
}

fn part02(instructions: Vec<Instruction>) {
    let mut cpu = CPU::default();
    cpu.load(instructions);

    let mut buffer = [false; 40];
    for cycle in 1..=240 {
        cpu.start();

        let pointer_position = (cycle - 1) % 40;
        let sprite = cpu.register_x - 1..=cpu.register_x + 1;

        if sprite.contains(&pointer_position) {
            buffer[pointer_position as usize] = true;
        }

        if cycle % 40 == 0 {
            for b in &mut buffer {
                print!("{}", if *b { '#' } else { '.' });
                *b = false;
            }
            println!();
        }

        cpu.stop();
    }
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    let reader = BufReader::new(File::open(&fpath).unwrap());
    let instructions: Vec<Instruction> = reader
        .lines()
        .map(Result::unwrap)
        .map(|l| Instruction::try_from(l.as_str()).unwrap())
        .collect();

    part01(instructions.clone());
    part02(instructions);
}
