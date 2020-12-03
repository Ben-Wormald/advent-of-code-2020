#[derive(Debug)]
struct EntryPartOne {
    lower: usize,
    upper: usize,
    character: char,
    password: Vec<char>,
}

#[derive(Debug)]
struct Entry {
    pos_1: usize,
    pos_2: usize,
    character: char,
    password: Vec<char>,
}

pub fn solve_part_one(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let bound_parts: Vec<usize> = parts[0].split('-').map(|part| part.parse().unwrap()).collect();
            let characters: Vec<char> = parts[1].chars().collect();
            
            EntryPartOne {
                lower: bound_parts[0],
                upper: bound_parts[1],
                character: characters[0],
                password: parts[2].chars().collect(),
            }
        })
        .filter(|entry| {
            let char_count = entry.password.iter().filter(|pw_char| **pw_char == entry.character).count();
            char_count >= entry.lower && char_count <= entry.upper
        })
        .count()
}

pub fn solve(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pos_parts: Vec<usize> = parts[0].split('-').map(|part| part.parse().unwrap()).collect();
            let characters: Vec<char> = parts[1].chars().collect();
            
            Entry {
                pos_1: pos_parts[0],
                pos_2: pos_parts[1],
                character: characters[0],
                password: parts[2].chars().collect(),
            }
        })
        .filter(|entry| {
            let pos_1_match = entry.password[entry.pos_1 - 1] == entry.character;
            let pos_2_match = entry.password[entry.pos_2 - 1] == entry.character;
            (pos_1_match && !pos_2_match) || (!pos_1_match && pos_2_match)
        })
        .count()
}
