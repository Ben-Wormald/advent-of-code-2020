use std::collections::HashMap;

const TURNS: usize = 30000000;

pub fn solve(input: &str) -> usize {
    let initial_numbers = process(input);
    let mut number_timestamps = HashMap::new();

    let mut previous_number: Option<usize> = None;

    for t in 0..initial_numbers.len() {
        match previous_number {
            None => (),
            Some(n) => { number_timestamps.insert(n, t); },
        }
        previous_number = Some(initial_numbers[t]);
    }

    let mut previous_number: usize = previous_number.unwrap();

    for t in initial_numbers.len()..TURNS {
        let number = match number_timestamps.contains_key(&previous_number) {
            true => t - number_timestamps.get(&previous_number).unwrap(),
            false => 0,
        };
        number_timestamps.insert(previous_number, t);
        previous_number = number;
    }
    
    previous_number
}

fn process(input: &str) -> Vec<usize> {
    let lines: Vec<&str> = input.lines().collect();
    lines[0].split(",").map(|x| x.parse().unwrap()).collect()
}
