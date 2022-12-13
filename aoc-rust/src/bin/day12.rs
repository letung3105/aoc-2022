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

fn dijkstra(adjacency_list: &[Vec<Vec<Position>>], start_position: Position) -> Vec<Vec<usize>> {
    let mut minimum_steps: Vec<Vec<usize>> = adjacency_list
        .iter()
        .map(|row| vec![usize::MAX; row.len()])
        .collect();
    minimum_steps[start_position.0][start_position.1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(DijkstraState {
        position: start_position,
        steps: 0,
    });

    while let Some(DijkstraState { position, steps }) = heap.pop() {
        if steps > minimum_steps[position.0][position.1] {
            // Skip since taking the current path takes a higher number of steps
            continue;
        }
        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for &neighbouring_position in &adjacency_list[position.0][position.1] {
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

    minimum_steps
}

fn part01(
    adjacency_list: &[Vec<Vec<Position>>],
    start_positioin: Position,
    end_position: Position,
) {
    let minimum_steps = dijkstra(adjacency_list, start_positioin);
    println!("{}", minimum_steps[end_position.0][end_position.1]);
}

fn part02(
    adjacency_list: &[Vec<Vec<Position>>],
    start_position: Position,
    end_positions: &[Position],
) {
    let minimum_steps = dijkstra(adjacency_list, start_position);
    let mut minimum_step = usize::MAX;
    for pos in end_positions {
        minimum_step = minimum_step.min(minimum_steps[pos.0][pos.1]);
    }
    println!("{}", minimum_step);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    let file = File::open(&fpath).unwrap();
    let reader = BufReader::new(file);

    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    let mut grid = Vec::default();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut grid_row = Vec::default();
        for (j, c) in line.trim().chars().enumerate() {
            let c = match c {
                'S' => {
                    start_position.0 = i;
                    start_position.1 = j;
                    'a'
                }
                'E' => {
                    end_position.0 = i;
                    end_position.1 = j;
                    'z'
                }
                c => c,
            };
            grid_row.push(c);
        }
        grid.push(grid_row);
    }

    let mut potential_starts = Vec::default();
    let mut adjacency_list_p1: Vec<Vec<Vec<Position>>> = grid
        .iter()
        .map(|row| row.iter().map(|_| Vec::default()).collect())
        .collect();
    let mut adjacency_list_p2: Vec<Vec<Vec<Position>>> = grid
        .iter()
        .map(|row| row.iter().map(|_| Vec::default()).collect())
        .collect();

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let current_position = (i, j);
            if c == 'a' {
                potential_starts.push(current_position);
            }

            let potential_neighbours = [
                (i.saturating_sub(1), j),
                (i.saturating_add(1).min(grid.len() - 1), j),
                (i, j.saturating_sub(1)),
                (i, j.saturating_add(1).min(row.len() - 1)),
            ];
            for pos in potential_neighbours {
                if pos == current_position {
                    continue;
                }
                let h1 = c as u32;
                let h2 = grid[pos.0][pos.1] as u32;
                if h2 <= h1 || h2 - h1 == 1 {
                    adjacency_list_p1[i][j].push(pos);
                }
                if h2 >= h1 || h1 - h2 == 1 {
                    adjacency_list_p2[i][j].push(pos);
                }
            }
        }
    }

    part01(&adjacency_list_p1, start_position, end_position);
    part02(&adjacency_list_p2, end_position, &potential_starts);
}
