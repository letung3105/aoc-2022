use std::{
    collections::HashSet,
    env,
    fmt::Formatter,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Item {
    val: u8,
    pri: u64,
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}-{}",
            char::from_u32(self.val.into()).unwrap(),
            self.pri
        )
    }
}

impl Item {
    fn new(c: u8) -> Self {
        let pri = if (b'a'..=b'z').contains(&c) {
            (1 + (c - b'a')).into()
        } else if (b'A'..=b'Z').contains(&c) {
            (27 + (c - b'A')).into()
        } else {
            panic!("unknown");
        };

        return Item { val: c, pri };
    }
}

fn part01(file: File) {
    let mut sum = 0;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line_length = line.len();
        let line_mid_pos = line_length / 2;

        let mut sack1 = HashSet::new();
        let mut sack2 = HashSet::new();
        for (i, c) in line.bytes().enumerate() {
            let item = Item::new(c);
            if i < line_mid_pos {
                sack1.insert(item);
            } else {
                sack2.insert(item);
            }
        }

        let inter = &sack1 & &sack2;
        for item in inter {
            sum += item.pri;
        }
    }

    println!("{}", sum);
}

fn part02(file: File) {
    let mut sum = 0;
    let mut sacks = vec![HashSet::new(), HashSet::new(), HashSet::new()];

    let reader = BufReader::new(file);
    for (line_no, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for c in line.bytes() {
            let item = Item::new(c);
            sacks[line_no % 3].insert(item);
        }

        if line_no % 3 == 2 {
            let mut iter = sacks.drain(..);
            let inter = iter
                .next()
                .map(|sack: HashSet<Item>| iter.fold(sack, |sack1, sack2| &sack1 & &sack2))
                .unwrap();
            for item in inter {
                sum += item.pri;
            }
            sacks.push(HashSet::new());
            sacks.push(HashSet::new());
            sacks.push(HashSet::new());
        }

    }

    println!("{}", sum);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    part01(File::open(&fpath).unwrap());
    part02(File::open(&fpath).unwrap());
}
