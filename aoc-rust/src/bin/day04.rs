use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct ElfPair(RangeInclusive<u64>, RangeInclusive<u64>);

fn parse_section(segment: &str) -> RangeInclusive<u64> {
    let mut ids = segment.split('-');
    let start = ids.next().unwrap().parse().unwrap();
    let end = ids.next().unwrap().parse().unwrap();
    start..=end
}

fn parse_elf_pair(line: &str) -> ElfPair {
    let mut sections = line.split(',');
    let s1 = parse_section(sections.next().unwrap());
    let s2 = parse_section(sections.next().unwrap());
    ElfPair(s1, s2)
}

fn is_fully_contains(s1: &RangeInclusive<u64>, s2: &RangeInclusive<u64>) -> bool {
    s1.contains(s2.start()) && s1.contains(s2.end())
}

fn is_overlap(s1: &RangeInclusive<u64>, s2: &RangeInclusive<u64>) -> bool {
    s1.contains(s2.start()) || s1.contains(s2.end())
}

fn part01(file: File) {
    let mut count = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let pair = parse_elf_pair(&line);
        if is_fully_contains(&pair.0, &pair.1) || is_fully_contains(&pair.1, &pair.0) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part02(file: File) {
    let mut count = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let pair = parse_elf_pair(&line);
        if is_overlap(&pair.0, &pair.1) || is_overlap(&pair.1, &pair.0) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    part01(File::open(&fpath).unwrap());
    part02(File::open(&fpath).unwrap());
}
