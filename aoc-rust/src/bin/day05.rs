use std::{
    collections::{BTreeMap, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug)]
enum Token {
    Crate(String),
    StackId(u32),
}

impl Token {
    fn parse(s: &str) -> Option<Self> {
        if let (Some(begin), Some(end)) = (s.find('['), s.find(']')) {
            return Some(Token::Crate(String::from(&s[begin + 1..end])));
        }
        s.trim().parse().ok().map(Self::StackId)
    }
}

#[derive(Debug)]
struct Inst {
    count: u64,
    from: u32,
    to: u32,
}

impl Inst {
    fn parse(s: &str) -> Option<Self> {
        let mut tokens = s.split_whitespace();

        tokens.next()?;
        let count = tokens.next()?.parse().ok()?;

        tokens.next()?;
        let from = tokens.next()?.parse().ok()?;

        tokens.next()?;
        let to = tokens.next()?.parse().ok()?;

        Some(Self { count, from, to })
    }
}

fn parse_state<R: BufRead>(lines: &mut Lines<R>) -> BTreeMap<u32, VecDeque<String>> {
    let mut stacks = Vec::default();
    let mut ids = Vec::default();
    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }
        for (idx, chunk) in line.as_bytes().chunks(4).enumerate() {
            let chunk = std::str::from_utf8(&chunk[0..3]).unwrap();
            if idx >= stacks.len() {
                stacks.push(VecDeque::default());
            }
            match Token::parse(chunk) {
                Some(Token::Crate(c)) => {
                    stacks[idx].push_front(c);
                }
                Some(Token::StackId(id)) => {
                    ids.push(id);
                }
                None => {}
            }
        }
    }
    let mut stacks_map = BTreeMap::new();
    for (idx, stack) in stacks.drain(..).enumerate() {
        stacks_map.insert(ids[idx], stack);
    }
    stacks_map
}

fn parse_steps<R: BufRead>(lines: &mut Lines<R>) -> Vec<Inst> {
    let mut steps = Vec::default();
    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }
        if let Some(inst) = Inst::parse(&line) {
            steps.push(inst);
        }
    }
    steps
}

fn part01(file: File) {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut state = parse_state(&mut lines);
    let steps = parse_steps(&mut lines);
    for step in steps {
        let mut items = Vec::default();
        let stack1 = state.get_mut(&step.from).unwrap();
        for _ in 0..step.count {
            if let Some(item) = stack1.pop_back() {
                items.push(item)
            }
        }
        drop(stack1);
        let stack2 = state.get_mut(&step.to).unwrap();
        for item in items {
            stack2.push_back(item);
        }
    }

    for (_, v) in state {
        print!("{}", v.back().unwrap());
    }
    println!();
}

fn part02(file: File) {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut state = parse_state(&mut lines);
    let steps = parse_steps(&mut lines);
    for step in steps {
        let stack1 = state.get_mut(&step.from).unwrap();
        let items = stack1.split_off(stack1.len() - step.count as usize);
        drop(stack1);
        let stack2 = state.get_mut(&step.to).unwrap();
        for item in items {
            stack2.push_back(item);
        }
    }

    for (_, v) in state {
        print!("{}", v.back().unwrap());
    }
    println!();
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    part01(File::open(&fpath).unwrap());
    part02(File::open(&fpath).unwrap());
}
