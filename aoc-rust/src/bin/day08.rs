use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    let reader = BufReader::new(File::open(&fpath).unwrap());

    // Read map

    let mut grid_tree_height = Vec::default();
    let mut grid_visibility = Vec::default();
    let mut grid_scenic_score = Vec::default();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row_tree_height = Vec::default();
        let mut row_visibility = Vec::default();
        let mut row_scenic_score = Vec::default();
        for c in line.chars() {
            row_tree_height.push(c.to_digit(10).unwrap());
            row_visibility.push(false);
            row_scenic_score.push(1u64);
        }
        grid_tree_height.push(row_tree_height);
        grid_visibility.push(row_visibility);
        grid_scenic_score.push(row_scenic_score);
    }

    // Part 01

    let height = grid_tree_height.len();
    let width = grid_tree_height[0].len();
    for i in 0..height {
        grid_visibility[i][0] = true;
        grid_visibility[i][width - 1] = true;
    }
    for j in 0..width {
        grid_visibility[0][j] = true;
        grid_visibility[height - 1][j] = true;
    }

    let mut highest_from_top = grid_tree_height[0].clone();
    let mut highest_from_left = Vec::with_capacity(height);
    let mut highest_from_bottom = grid_tree_height[height - 1].clone();
    let mut highest_from_right = Vec::with_capacity(height);
    for i in 0..height {
        highest_from_left.push(grid_tree_height[i][0]);
        highest_from_right.push(grid_tree_height[i][width - 1]);
    }

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let visible_top = grid_tree_height[i][j] > highest_from_top[j];
            let visible_left = grid_tree_height[i][j] > highest_from_left[i];
            grid_visibility[i][j] |= visible_top || visible_left;

            highest_from_top[j] = highest_from_top[j].max(grid_tree_height[i][j]);
            highest_from_left[i] = highest_from_left[i].max(grid_tree_height[i][j]);
        }
    }
    for i in (1..height - 1).rev() {
        for j in (1..width - 1).rev() {
            let visible_bottom = grid_tree_height[i][j] > highest_from_bottom[j];
            let visible_right = grid_tree_height[i][j] > highest_from_right[i];
            grid_visibility[i][j] |= visible_bottom || visible_right;

            highest_from_bottom[j] = highest_from_bottom[j].max(grid_tree_height[i][j]);
            highest_from_right[i] = highest_from_right[i].max(grid_tree_height[i][j]);
        }
    }

    for (row_visibility, row_height) in grid_visibility.iter().zip(grid_tree_height.iter()) {
        for (v, h) in row_visibility.iter().zip(row_height.iter()) {
            print!("{}-{:<3}", h, if *v { 'T' } else { 'F' });
        }
        println!();
    }
    let count = grid_visibility.iter().flatten().filter(|x| **x).count();
    println!("{}", count);

    // Part 02

    let mut last_known_idx_top = Vec::default();
    let mut last_known_idx_left = Vec::default();
    let mut last_known_idx_bottom = Vec::default();
    let mut last_known_idx_right = Vec::default();
    for _ in 0..height {
        last_known_idx_left.push(vec![0usize; 10]);
        last_known_idx_right.push(vec![width - 1; 10]);
    }
    for _ in 0..width {
        last_known_idx_top.push(vec![0usize; 10]);
        last_known_idx_bottom.push(vec![height - 1; 10]);
    }

    for i in 0..height {
        for j in 0..width {
            let height = grid_tree_height[i][j] as usize;
            grid_scenic_score[i][j] *= (i - last_known_idx_top[j][height]) as u64;
            grid_scenic_score[i][j] *= (j - last_known_idx_left[i][height]) as u64;
            for t in 1..=height {
                last_known_idx_top[j][t] = i;
                last_known_idx_left[i][t] = j;
            }
        }
    }
    for i in (0..height).rev() {
        for j in (0..width).rev() {
            let height = grid_tree_height[i][j] as usize;
            grid_scenic_score[i][j] *= (last_known_idx_bottom[j][height] - i) as u64;
            grid_scenic_score[i][j] *= (last_known_idx_right[i][height] - j) as u64;
            for t in 0..=height {
                last_known_idx_bottom[j][t] = i;
                last_known_idx_right[i][t] = j;
            }
        }
    }

    for (row_scenic_score, row_height) in grid_scenic_score.iter().zip(grid_tree_height.iter()) {
        for (s, h) in row_scenic_score.iter().zip(row_height.iter()) {
            print!("{}-{:<3}", h, *s);
        }
        println!();
    }
    let max_score = grid_scenic_score.iter().flatten().max().unwrap();
    println!("{}", max_score);
}
