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

fn dijkstra(
    adjacency_list: &[Vec<Vec<(usize, usize)>>],
    position_start: Position,
    position_end: Position,
) -> usize {
    let mut minimum_steps: Vec<Vec<usize>> = adjacency_list
        .iter()
        .map(|row| vec![usize::MAX; row.len()])
        .collect();
    minimum_steps[position_start.0][position_start.1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(DijkstraState {
        position: position_start,
        steps: 0,
    });

    while let Some(DijkstraState { position, steps }) = heap.pop() {
        if position.0 == position_end.0 && position.1 == position_end.1 {
            // We reach the destination
            break;
        }
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

    minimum_steps[position_end.0][position_end.1]
}

fn part01(
    adjacency_list: &[Vec<Vec<(usize, usize)>>],
    position_start: Position,
    position_end: Position,
) {
    let minimum_step = dijkstra(adjacency_list, position_start, position_end);
    println!("{}", minimum_step);
}

fn part02(
    adjacency_list: &[Vec<Vec<(usize, usize)>>],
    potential_starts: &[(usize, usize)],
    position_end: Position,
) {
    let mut minimum_step = usize::MAX;
    for &position_start in potential_starts {
        minimum_step = minimum_step.min(dijkstra(adjacency_list, position_start, position_end));
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

    let mut potential_starts = Vec::default();
    let mut adjacency_list: Vec<Vec<Vec<(usize, usize)>>> = grid
        .iter()
        .map(|row| row.iter().map(|_| Vec::default()).collect())
        .collect();

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'a' {
                potential_starts.push((i, j));
            }
            // Find neighbours that can be visited
            adjacency_list[i][j].extend(
                [
                    (i.saturating_sub(1), j),
                    (i.saturating_add(1).min(grid.len() - 1), j),
                    (i, j.saturating_sub(1)),
                    (i, j.saturating_add(1).min(row.len() - 1)),
                ]
                .into_iter()
                .filter(|&pos| {
                    if i == pos.0 && j == pos.1 {
                        return false;
                    }
                    let h1 = c as u32;
                    let h2 = grid[pos.0][pos.1] as u32;
                    h2 <= h1 || h2 - h1 == 1
                }),
            );
        }
    }

    part01(&adjacency_list, position_start, position_end);
    part02(&adjacency_list, &potential_starts, position_end);
}
