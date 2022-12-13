use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Singular(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_list = |xs: &[Packet], ys: &[Packet]| {
            for (x, y) in xs.iter().zip(ys.iter()) {
                match x.cmp(y) {
                    Ordering::Equal => {}
                    ordering => return ordering,
                }
            }
            xs.len().cmp(&ys.len())
        };
        match (self, other) {
            (Packet::Singular(n), Packet::Singular(m)) => n.cmp(m),
            (Packet::List(n), Packet::List(m)) => cmp_list(n, m),
            (Packet::Singular(n), Packet::List(m)) => cmp_list(&[Packet::Singular(*n)], m),
            (Packet::List(n), Packet::Singular(m)) => cmp_list(n, &[Packet::Singular(*m)]),
        }
    }
}

impl TryFrom<&str> for Packet {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut stack = Vec::default();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.peek() {
            match c {
                ',' => {
                    chars.next().ok_or("invalid state")?;
                }
                '[' => {
                    chars.next().ok_or("invalid state")?;
                    stack.push(Vec::default());
                }
                ']' => {
                    chars.next().ok_or("invalid state")?;
                    let nested = stack.pop().ok_or("invalid state")?;
                    match stack.last_mut() {
                        None => return Ok(Packet::List(nested)),
                        Some(last) => last.push(Packet::List(nested)),
                    }
                }
                _ => {
                    let mut value = 0;
                    while let Some(c) = chars.peek() {
                        match c.to_digit(10) {
                            None => break,
                            Some(n) => {
                                value *= 10;
                                value += n as usize;
                                chars.next();
                            }
                        };
                    }
                    let last = stack.last_mut().ok_or("invalid state")?;
                    last.push(Packet::Singular(value));
                }
            }
        }
        Err("invalid state")
    }
}

fn part01(pairs: Vec<(Packet, Packet)>) {
    let mut result = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if let Ordering::Less = pair.0.cmp(&pair.1) {
            result += i + 1;
        }
    }
    println!("{}", result);
}

fn part02(pairs: Vec<(Packet, Packet)>) {
    let divider01 = Packet::List(vec![Packet::List(vec![Packet::Singular(2)])]);
    let divider02 = Packet::List(vec![Packet::List(vec![Packet::Singular(6)])]);

    let mut packets: Vec<Packet> = pairs.into_iter().map(|p| [p.0, p.1]).flatten().collect();
    packets.push(divider01.clone());
    packets.push(divider02.clone());
    packets.sort();

    let mut result = 1;
    for (i, packet) in packets.iter().enumerate() {
        if *packet == divider01 || *packet == divider02 {
            result *= i + 1;
        }
    }
    println!("{}", result);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    let file = File::open(&fpath).unwrap();
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines().map(Result::unwrap).peekable();
    let mut pairs = Vec::default();

    while let Some(line) = lines_iter.peek() {
        if line.is_empty() {
            lines_iter.next();
            continue;
        }
        let l = Packet::try_from(lines_iter.next().unwrap().as_str()).unwrap();
        let r = Packet::try_from(lines_iter.next().unwrap().as_str()).unwrap();
        pairs.push((l, r));
    }

    part01(pairs.clone());
    part02(pairs);
}
