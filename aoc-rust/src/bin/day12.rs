use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

type Position = (usize, usize);

#[derive(PartialEq, Eq)]
struct DijkstraState {
    position: Position,
    steps: usize,
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&other.steps)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn dijkstra(grid: &[Vec<char>], position_start: Position, position_end: Position) -> usize {
    let height = grid.len();
    let width = grid.iter().map(Vec::len).min().unwrap();

    let mut minimum_steps: Vec<Vec<usize>> = (0..height).map(|_| vec![usize::MAX; width]).collect();
    minimum_steps[position_start.0][position_start.1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(DijkstraState {
        position: position_start,
        steps: 0,
    });

    while let Some(DijkstraState { position, steps }) = heap.pop() {
        if steps > minimum_steps[position.0][position.1] {
            // Skip since taking the current path takes a higher number of steps
            continue;
        }

        // Find neighbours that can be visited
        let neighbouring_positions: Vec<Position> = [
            (position.0.saturating_sub(1), position.1),
            (position.0.saturating_add(1).min(height - 1), position.1),
            (position.0, position.1.saturating_sub(1)),
            (position.0, position.1.saturating_add(1).min(width - 1)),
        ]
        .into_iter()
        .filter(|&neighbouring_position| {
            if position.0 == neighbouring_position.0 && position.1 == neighbouring_position.1 {
                return false;
            }
            let h1 = grid[position.0][position.1] as u32;
            let h2 = grid[neighbouring_position.0][neighbouring_position.1] as u32;
            h2 <= h1 || h2 - h1 == 1
        })
        .collect();

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbouring_position in neighbouring_positions {
            let next = DijkstraState {
                steps: steps + 1,
                position: neighbouring_position,
            };
            if next.steps < minimum_steps[next.position.0][next.position.1] {
                // We have now found a better way
                minimum_steps[next.position.0][next.position.1] = next.steps;
                heap.push(next);
            }
        }
    }

    minimum_steps[position_end.0][position_end.1]
}

fn part01(grid: &[Vec<char>], position_start: Position, position_end: Position) {
    let minimum_step = dijkstra(grid, position_start, position_end);
    println!("{}", minimum_step);
}

fn part02(grid: &[Vec<char>], position_end: Position) {
    let mut starting_positions = Vec::default();
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'a' {
                starting_positions.push((i, j));
            }
        }
    }
    let mut minimum_step = usize::MAX;
    for position_start in starting_positions {
        minimum_step = minimum_step.min(dijkstra(grid, position_start, position_end));
    }
    println!("{}", minimum_step);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    let file = File::open(&fpath).unwrap();
    let reader = BufReader::new(file);

    let mut position_start = (0, 0);
    let mut position_end = (0, 0);
    let mut grid = Vec::default();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut grid_row = Vec::default();
        for (j, c) in line.trim().chars().enumerate() {
            let c = match c {
                'S' => {
                    position_start.0 = i;
                    position_start.1 = j;
                    'a'
                }
                'E' => {
                    position_end.0 = i;
                    position_end.1 = j;
                    'z'
                }
                c => c,
            };
            grid_row.push(c);
        }
        grid.push(grid_row);
    }

    part01(&grid, position_start, position_end);
    part02(&grid, position_end);
}
