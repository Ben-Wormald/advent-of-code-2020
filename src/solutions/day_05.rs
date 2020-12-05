pub fn solve_part_one(input: &str) -> usize {
    let mut max = 0;

    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .for_each(|pass| {
            let seat_id = get_seat_id(pass);
            if seat_id > max {
                max = seat_id;
            }
        });

    max
}

fn get_seat_id(pass: &str) -> usize {
    let mut row_sum = 0;
    let mut row_bit = 64;
    pass[..7].chars().for_each(|character| {
        if character == 'B' {
            row_sum += row_bit;
        }
        row_bit /= 2;
    });

    let mut col_sum = 0;
    let mut col_bit = 4;
    pass[7..].chars().for_each(|character| {
        if character == 'R' {
            col_sum += col_bit;
        }
        col_bit /= 2;
    });

    row_sum * 8 + col_sum
}

pub fn solve(input: &str) -> usize {
    let mut found = [false; 1024];

    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .for_each(|pass| {
            let seat_id = get_seat_id(pass);
            found[seat_id - 1] = true;
        });

    let mut missing = 0;
    for i in 1..(found.len() - 2) {
        if !found[i] && found[i - 1] && found[i + 1] {
            missing = i;
        }
    }
    missing + 1
}
