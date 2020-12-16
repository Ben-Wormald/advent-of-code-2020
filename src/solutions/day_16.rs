#[derive(Debug)]
struct Field {
    name: String,
    range_a: Range,
    range_b: Range,
}

#[derive(Debug)]
struct Range {
    lower: usize,
    upper: usize,
}

pub fn solve(input: &str) -> usize {
    let (fields, _, tickets) = process(input);
    
    tickets
        .iter()
        .fold(0, |error_sum, ticket| {
            error_sum + get_error(&fields, ticket)
        })
}

fn process(input: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let fields = parts[0].lines().map(|field| {
        let field_parts: Vec<&str> = field.split(": ").collect();
        let ranges_parts: Vec<&str> = field_parts[1].split(" or ").collect();
        let range_a_parts: Vec<&str> = ranges_parts[0].split("-").collect();
        let range_b_parts: Vec<&str> = ranges_parts[1].split("-").collect();

        Field {
            name: field_parts[0].to_string(),
            range_a: Range {
                lower: range_a_parts[0].parse().unwrap(),
                upper: range_a_parts[1].parse().unwrap(),
            },
            range_b: Range {
                lower: range_b_parts[0].parse().unwrap(),
                upper: range_b_parts[1].parse().unwrap(),
            },
        }
    }).collect();

    let my_ticket: Vec<&str> = parts[1].lines().collect();
    let my_ticket = my_ticket[1]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let tickets: Vec<&str> = parts[2].lines().collect();
    let tickets = tickets[1..]
        .iter()
        .map(|ticket| ticket
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect()
        )
        .collect();

    (fields, my_ticket, tickets)
}

fn get_error(fields: &Vec<Field>, ticket: &Vec<usize>) -> usize {
    ticket
        .iter()
        .fold(0, |error_sum, value| {
            let valid = fields
                .iter()
                .fold(false, |valid, field| {
                    let in_range_a = value >= &field.range_a.lower && value <= &field.range_a.upper;
                    let in_range_b = value >= &field.range_b.lower && value <= &field.range_b.upper;

                    if in_range_a || in_range_b {
                        true
                    }
                    else {
                        valid
                    }
                });

            match valid {
                true => error_sum,
                false => error_sum + value,
            }
        })
}
