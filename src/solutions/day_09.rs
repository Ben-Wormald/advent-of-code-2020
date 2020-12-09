const PREAMBLE_LEN: usize = 25;

pub fn solve_part_one(input: &str) -> usize {
    let data = process_input(input);

    let mut search_range = [0; PREAMBLE_LEN];
    for n in 0..PREAMBLE_LEN {
        search_range[n] = data[n];
    }

    let mut result: Option<usize> = None;
    for ptr in PREAMBLE_LEN..data.len() {
        let mut found = false;
        for i in 0..search_range.len() {
            for j in 0..search_range.len() {
                if i != j && search_range[i] + search_range[j] == data[ptr] {
                    found = true;
                    break
                }
            }
            if found { break }
        }
        if !found {
            result = Some(data[ptr]);
            break
        }

        search_range[ptr % PREAMBLE_LEN] = data[ptr];
    }

    match result {
        Some(value) => value,
        None => 0,
    }
}

fn process_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn solve(input: &str) -> usize {
    let data = process_input(input);
    let target = solve_part_one(input);

    let mut range: Option<(usize, usize)> = None;

    for start in 0..data.len() {
        let mut sum = data[start];
        let mut end = start + 1;

        while sum < target {
            sum += data[end];
            end += 1;
        }

        if sum == target {
            range = Some((start, end));
            break
        }
    }

    match range {
        Some((start, end)) => sum_extremes(&data[start..end]),
        None => 0
    }
}

fn sum_extremes(range: &[usize]) -> usize {
    match (range.iter().min(), range.iter().max()) {
        (Some(min), Some(max)) => min + max,
        (_, _) => 0
    }
}
