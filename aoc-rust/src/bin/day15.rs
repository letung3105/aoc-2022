use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

type Position = (isize, isize);

fn parse_postion(s: &str) -> Position {
    let mut parts = s.split(',');
    let x_part = parts.next().unwrap().trim();
    let y_part = parts.next().unwrap().trim();
    let x: isize = x_part.strip_prefix("x=").unwrap().parse().unwrap();
    let y: isize = y_part.strip_prefix("y=").unwrap().parse().unwrap();
    (x, y)
}

fn manhattan_distance(p1: &Position, p2: &Position) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn get_line_coverage(
    sensors: &HashMap<Position, usize>,
    begin: isize,
    end: isize,
) -> HashMap<isize, Vec<RangeInclusive<isize>>> {
    let mut line_coverage = HashMap::default();
    for y in begin..=end {
        let mut ranges = Vec::default();
        for (sensor, distance) in sensors {
            let y_diff = sensor.1.abs_diff(y);
            if y_diff > *distance {
                continue;
            }
            let x_diff = distance.abs_diff(y_diff);
            let range = sensor.0 - x_diff as isize..=sensor.0 + x_diff as isize;
            ranges.push(range);
        }

        if ranges.is_empty() {
            continue;
        }
        ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));

        let mut merged_ranges = Vec::with_capacity(ranges.len());
        for range in ranges {
            match merged_ranges.last_mut() {
                None => merged_ranges.push(range),
                Some(merged_range) => {
                    if merged_range.end() >= range.start() {
                        let merged_range_end = *merged_range.end().max(range.end());
                        *merged_range = *merged_range.start()..=merged_range_end;
                    } else {
                        merged_ranges.push(range)
                    }
                }
            }
        }
        line_coverage.insert(y, merged_ranges);
    }
    line_coverage
}

fn part01(
    beacons: &HashSet<Position>,
    line_coverage: &HashMap<isize, Vec<RangeInclusive<isize>>>,
    line: isize,
) -> usize {
    let mut count = 0;
    if let Some(ranges) = line_coverage.get(&line) {
        for range in ranges {
            count += range
                .clone()
                .filter(|i| !beacons.contains(&(*i, line)))
                .count();
        }
    }
    count
}

fn part02(
    line_coverage: &HashMap<isize, Vec<RangeInclusive<isize>>>,
    begin: isize,
    end: isize,
) -> isize {
    for y in begin..=end {
        if let Some(ranges) = line_coverage.get(&y) {
            if ranges.len() == 2 {
                let x = ranges[0].end() + 1;
                return x * 4000000 + y;
            }
        }
    }
    0
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    let file = File::open(&fpath).unwrap();
    let reader = BufReader::new(file);

    let mut sensors: HashMap<Position, usize> = HashMap::default();
    let mut beacons: HashSet<Position> = HashSet::default();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_parts = line.split(':');
        let loc_sensor = line_parts
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Sensor at")
            .unwrap()
            .trim();
        let loc_beacon = line_parts
            .next()
            .unwrap()
            .trim()
            .strip_prefix("closest beacon is at")
            .unwrap()
            .trim();
        let sensor = parse_postion(loc_sensor);
        let beacon = parse_postion(loc_beacon);
        let distance = manhattan_distance(&sensor, &beacon);
        sensors.insert(sensor, distance);
        beacons.insert(beacon);
    }

    let ex01 = part01(&beacons, &get_line_coverage(&sensors, 10, 10), 10);
    println!("{}", ex01);

    let res01 = part01(
        &beacons,
        &get_line_coverage(&sensors, 2000000, 2000000),
        2000000,
    );
    println!("{}", res01);

    let ex02 = part02(&get_line_coverage(&sensors, 0, 20), 0, 20);
    println!("{}", ex02);

    let res02 = part02(&get_line_coverage(&sensors, 0, 4000000), 0, 4000000);
    println!("{}", res02);
}
