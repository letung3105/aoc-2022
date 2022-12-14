use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

const SAND_STARTING_POSTION: Position = (500, 0);

type Position = (usize, usize);

fn simulate01(mut blockages: HashSet<Position>) -> Vec<Position> {
    let mut sands = vec![SAND_STARTING_POSTION];
    let y_max = blockages.iter().map(|s| s.1).max().unwrap();
    loop {
        let current_sand = sands.last_mut().unwrap();

        if current_sand.1 >= y_max {
            // Sand falls forever
            sands.pop();
            break sands;
        }

        let y_down = current_sand.1 + 1;
        if !blockages.contains(&(current_sand.0, y_down)) {
            current_sand.1 = y_down;
            continue;
        }

        let x_left = current_sand.0 - 1;
        if !blockages.contains(&(x_left, y_down)) {
            current_sand.0 = x_left;
            current_sand.1 = y_down;
            continue;
        }

        let x_right = current_sand.0 + 1;
        if !blockages.contains(&(x_right, y_down)) {
            current_sand.0 = x_right;
            current_sand.1 = y_down;
            continue;
        }

        // Sand is blocked
        blockages.insert(*current_sand);
        sands.push(SAND_STARTING_POSTION);
    }
}

fn simulate02(mut blockages: HashSet<Position>) -> Vec<Position> {
    let mut sands = vec![SAND_STARTING_POSTION];
    let y_max = blockages.iter().map(|s| s.1).max().unwrap() + 1;
    loop {
        let current_sand = sands.last_mut().unwrap();

        if current_sand.1 >= y_max {
            // Reached the max depth
            blockages.insert(*current_sand);
            sands.push(SAND_STARTING_POSTION);
            continue;
        }

        let y_down = current_sand.1 + 1;
        if !blockages.contains(&(current_sand.0, y_down)) {
            current_sand.1 = y_down;
            continue;
        }

        let x_left = current_sand.0 - 1;
        if !blockages.contains(&(x_left, y_down)) {
            current_sand.0 = x_left;
            current_sand.1 = y_down;
            continue;
        }

        let x_right = current_sand.0 + 1;
        if !blockages.contains(&(x_right, y_down)) {
            current_sand.0 = x_right;
            current_sand.1 = y_down;
            continue;
        }

        if *current_sand == SAND_STARTING_POSTION {
            // Starting position is blocked
            break sands;
        }

        blockages.insert(*current_sand);
        sands.push(SAND_STARTING_POSTION);
    }
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    let file = File::open(&fpath).unwrap();
    let reader = BufReader::new(file);

    let mut blockages: HashSet<Position> = HashSet::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut previous_position: Option<Position> = None;
        for loc in line.split("->") {
            let mut loc = loc.trim().split(',');
            let new_position = (
                loc.next().unwrap().parse().unwrap(),
                loc.next().unwrap().parse().unwrap(),
            );

            let prev_position = previous_position.unwrap_or(new_position);
            let x_begin = prev_position.0.min(new_position.0);
            let x_end = prev_position.0.max(new_position.0);
            let y_begin = prev_position.1.min(new_position.1);
            let y_end = prev_position.1.max(new_position.1);
            previous_position.replace(new_position);

            for x in x_begin..=x_end {
                for y in y_begin..=y_end {
                    blockages.insert((x, y));
                }
            }
        }
    }

    let sands01 = simulate01(blockages.clone());
    println!("len={}", sands01.len());

    let sands02 = simulate02(blockages);
    println!("len={}", sands02.len());
}
