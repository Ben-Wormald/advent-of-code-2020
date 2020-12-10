use std::cmp;

pub fn solve_part_one(input: &str) -> usize {
    let mut adapters: Vec<usize> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    adapters.sort();
    
    let mut diff_1_count = 0;
    let mut diff_3_count = 1;

    let mut prev_joltage = 0;

    for i in 0..adapters.len() {
        if adapters[i] - prev_joltage == 1 {
            diff_1_count += 1;
        }
        else if adapters[i] - prev_joltage == 3 {
            diff_3_count += 1;
        }
        prev_joltage = adapters[i];
    }

    diff_1_count * diff_3_count
}

#[derive(Debug)]
struct Path {
    joltage: usize,
    path_count: usize,
}

fn process(input: &str) -> Vec<Path> {
    let mut joltages: Vec<usize> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    joltages.sort();
    joltages.insert(0, 0);

    joltages
        .into_iter()
        .map(|joltage| Path {
            joltage,
            path_count: if joltage == 0 { 1 } else { 0 },
        })
        .collect()
}

pub fn solve(input: &str) -> usize {
    let mut paths = process(input);

    for current in 0..paths.len() {
        for reachable in (current + 1)..cmp::min(current + 4, paths.len()) {
            if paths[reachable].joltage - paths[current].joltage <= 3 {
                paths[reachable].path_count = paths[reachable].path_count + paths[current].path_count;
            }
        }
    }

    paths.last().unwrap().path_count
}
