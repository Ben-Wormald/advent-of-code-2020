pub fn solve_part_one(input: &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();

    for line_a in &lines {
        if line_a == &"" { continue }
        let entry_a: u32 = line_a.parse().expect("oh no");

        for line_b in &lines {
            if line_b == &"" { continue }
            let entry_b: u32 = line_b.parse().expect("oh no");

            if entry_a + entry_b == 2020 {
                return entry_a * entry_b;
            }
        }
    }
    0
}

pub fn solve(input: &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();

    for line_a in &lines {
        if line_a == &"" { continue }
        let entry_a: u32 = line_a.parse().expect("oh no");

        for line_b in &lines {
            if line_b == &"" { continue }
            let entry_b: u32 = line_b.parse().expect("oh no");

            for line_c in &lines {
                if line_c == &"" { continue }
                let entry_c: u32 = line_c.parse().expect("oh no");
                
                if entry_a + entry_b + entry_c == 2020 {
                    return entry_a * entry_b * entry_c;
                }
            }
        }
    }
    0
}
