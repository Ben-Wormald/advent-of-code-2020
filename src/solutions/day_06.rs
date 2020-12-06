const QUESTIONS: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
    "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
];

pub fn solve_part_one(input: &str) -> usize {
    let mut sum = 0;
    input
        .split("\n\n")
        .for_each(|group| {
            QUESTIONS.iter().for_each(|question| {
                if group.contains(question) {
                    sum += 1;
                }
            })
        });
    sum
}

pub fn solve(input: &str) -> usize {
    let mut sum = 0;
    input
        .split("\n\n")
        .for_each(|group| {
            let passengers: Vec<&str> = group.split('\n').filter(|line| line.len() > 0).collect();

            QUESTIONS.iter().for_each(|question| {
                if passengers.iter().all(|passenger| passenger.contains(question)) {
                    sum += 1;
                }
            })
        });
    sum
}
