use regex::Regex;

const REQUIRED_FIELDS: [&str; 7] = [
    "byr:",
    "iyr:",
    "eyr:",
    "hgt:",
    "hcl:",
    "ecl:",
    "pid:",
];

pub fn solve_part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|line|
            REQUIRED_FIELDS.iter().all(|&field|
                line.contains(field)
            )
        )
        .count()
}

pub fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|line|
            REQUIRED_FIELDS.iter().all(|&field|
                line.contains(field)
            )
        )
        .filter(|line| {
            line.split_whitespace().all(|field| {
                let key_value: Vec<&str> = field.split(':').collect();
                validate(key_value[0], key_value[1])
            })
        })
        .count()
}

fn validate(key: &str, value: &str) -> bool {
    // compiling these every time is slow - should be pulled out elsewhere
    let hcl_regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();

    match key {
        "byr" => validate_range(value, 1920, 2002),
        "iyr" => validate_range(value, 2010, 2020),
        "eyr" => validate_range(value, 2020, 2030),
        "hgt" => validate_height(value),
        "hcl" => hcl_regex.is_match(value),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => pid_regex.is_match(value),
        "cid" => true,
        _ => false,
    }
}

fn validate_range(value: &str, lower: usize, upper: usize) -> bool {
    match value.parse::<usize>() {
        Ok(n) => n >= lower && n <= upper,
        _ => false,
    }
}

fn validate_height(value: &str) -> bool {
    let split_at = value.len() - 2;
    match &value[split_at..] {
        "cm" => validate_range(&value[..split_at], 150, 193),
        "in" => validate_range(&value[..split_at], 59, 76),
        _ => false,
    }
}
