use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse_input<P>(path: P) -> (HashMap<String, u32>, HashMap<String, Vec<String>>)
where
    P: AsRef<Path>,
{
    let mut rates = HashMap::<String, u32>::default();
    let mut nexts = HashMap::<String, Vec<String>>::default();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines().map(Result::unwrap) {
        let mut line = line.split_whitespace().enumerate().filter_map(|(idx, v)| {
            if idx > 8 {
                Some(v.trim_end_matches(','))
            } else if idx == 4 {
                Some(v.trim_start_matches("rate=").trim_end_matches(';'))
            } else if idx == 1 {
                Some(v)
            } else {
                None
            }
        });

        let valve = line.next().unwrap();
        let rate = line.next().unwrap().parse().unwrap();
        let next = line.map(str::to_string).collect();

        rates.insert(valve.to_string(), rate);
        nexts.insert(valve.to_string(), next);
    }

    (rates, nexts)
}

#[derive(PartialEq, Eq)]
struct DijkstraState<'a> {
    node: &'a str,
    cost: u32,
}

impl<'a> PartialOrd for DijkstraState<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for DijkstraState<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&other.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

fn dijkstra<'a>(nexts: &'a HashMap<String, Vec<String>>, start: &'a str) -> HashMap<&'a str, u32> {
    let mut costs: HashMap<&str, u32> = HashMap::default();
    let mut heap = BinaryHeap::new();

    costs.insert(start, 0);
    heap.push(DijkstraState {
        node: start,
        cost: 0,
    });

    while let Some(DijkstraState { node, cost }) = heap.pop() {
        if let Some(true) = costs.get(node).map(|c| cost > *c) {
            // Skip since taking the current path takes a higher number of steps
            continue;
        }
        // For each valve we can reach, see if we can find another path with a lower cost going through it
        for valve in &nexts[node] {
            let next = DijkstraState {
                node: valve.as_str(),
                cost: cost + 1,
            };
            match costs.get_mut(next.node) {
                None => {
                    costs.insert(next.node, next.cost);
                }
                Some(cost) => {
                    if next.cost < *cost {
                        *cost = next.cost;
                    }
                }
            }
            heap.push(next);
        }
    }
    costs
}

fn simulate<'a>(
    max_duration: u32,
    working_valves: &HashMap<&'a str, usize>,
    rates: &HashMap<String, u32>,
    travel_time: &HashMap<&'a str, HashMap<&'a str, u32>>,
) -> Vec<HashMap<&'a str, HashMap<usize, u32>>> {
    let mut timeline: Vec<HashMap<&str, HashMap<usize, u32>>> =
        vec![HashMap::default(); max_duration as usize];
    timeline[0].entry("AA").or_default().insert(0, 0);

    for instant in 0..timeline.len() - 1 {
        // Split the state vector into 2, so we can reference past and future states
        // at the same time
        let timeline_offset = instant + 1;
        let (past_timeline, future_timeline) = timeline.split_at_mut(timeline_offset);
        if past_timeline.is_empty() || future_timeline.is_empty() {
            continue;
        }
        for (src_valve, max_pressures) in past_timeline.last().unwrap().iter() {
            for (opened_bitmask, max_pressure) in max_pressures {
                for (dst_valve, dst_cost) in &travel_time[*src_valve] {
                    // Stay in-place and do nothing
                    let duration_passed = instant + 1;
                    future_timeline[duration_passed as usize - timeline_offset]
                        .entry(src_valve)
                        .or_default()
                        .entry(*opened_bitmask)
                        .and_modify(|pressure| {
                            *pressure = (*pressure).max(*max_pressure);
                        })
                        .or_insert(*max_pressure);

                    let dst_valve_id = working_valves[*dst_valve];
                    let dst_valve_bitmask = 1 << dst_valve_id;
                    if opened_bitmask & dst_valve_bitmask != 0 {
                        // Valve has been opened
                        continue;
                    }
                    let duration_passed = instant as u32 + dst_cost + 1;
                    if duration_passed >= max_duration {
                        // No time left to move to the valve
                        continue;
                    }

                    let rate = rates[*dst_valve];
                    let duration_remain = max_duration - duration_passed;
                    let new_opened_bitmask = opened_bitmask | dst_valve_bitmask;
                    let new_pressure = max_pressure + duration_remain * rate;

                    // Move to another valve and turn it on
                    future_timeline[duration_passed as usize - timeline_offset]
                        .entry(*dst_valve)
                        .or_default()
                        .entry(new_opened_bitmask)
                        .and_modify(|pressure| {
                            *pressure = (*pressure).max(new_pressure);
                        })
                        .or_insert(new_pressure);
                }
            }
        }
    }
    timeline
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");

    // Parse the input to get
    // + a mapping from each value to its flow rate
    // + a mapping from each valve to the valves next to it
    let (rates, nexts) = parse_input(fpath);

    // Calculate the costs of the shortest paths between a valve and every other valve
    let travel_time: HashMap<&str, HashMap<&str, u32>> = rates
        .keys()
        .map(|src| {
            // Only consider valves with flow rate > 0 as destinations
            let costs: HashMap<&str, u32> = dijkstra(&nexts, src)
                .into_iter()
                .filter(|(dst, _)| rates[*dst] > 0)
                .collect();
            (src.as_str(), costs)
        })
        .collect();

    // Assign each working valve a sequence number
    let working_valves: HashMap<&str, usize> = rates
        .iter()
        .filter_map(|(valve, rate)| if *rate == 0 { None } else { Some(valve) })
        .enumerate()
        .map(|(idx, valve)| (valve.as_str(), idx))
        .collect();

    let timeline01 = simulate(30, &working_valves, &rates, &travel_time);
    let part01 = timeline01[29]
        .iter()
        .flat_map(|(_, v)| v.values())
        .max()
        .unwrap();

    let mut part02 = 0;
    let timeline02 = simulate(26, &working_valves, &rates, &travel_time);
    for (mask01, pressure01) in timeline02[25].iter().flat_map(|(_, v)| v.iter()) {
        for (mask02, pressure02) in timeline02[25].iter().flat_map(|(_, v)| v.iter()) {
            if mask01 == mask02 {
                continue;
            }
            let intersection = mask01 & mask02;
            if intersection != 0 {
                continue;
            }
            let pressure = pressure01 + pressure02;
            part02 = part02.max(pressure);
        }
    }

    println!("{}", part01);
    println!("{}", part02);
}
