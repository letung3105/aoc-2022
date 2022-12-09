use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn normalize_alphabetic_ascii(b: u8) -> usize {
    (b - b'a') as usize
}

fn has_repetition(chunk: &[u8]) -> bool {
    let mut histogram = [0usize; 64];
    for b in chunk {
        let count = &mut histogram[normalize_alphabetic_ascii(*b)];
        *count += 1;
        if *count == 2 {
            return true;
        }
    }
    false
}

fn part01(file: File) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let result = line
            .as_bytes()
            .windows(4)
            .position(|chunk| !has_repetition(chunk))
            .map(|i| i + 4);
        if let Some(i) = result {
            println!("{}", i);
        }
    }
}

fn part02(file: File) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let result = line
            .as_bytes()
            .windows(14)
            .position(|chunk| !has_repetition(chunk))
            .map(|i| i + 14);
        if let Some(i) = result {
            println!("{}", i);
        }
    }
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    part01(File::open(&fpath).unwrap());
    part02(File::open(&fpath).unwrap());
}
